// SPDX-License-Identifier: Apache-2.0

use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_case_files(directory: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(directory).expect("schema directory should be readable") {
        let path = entry.expect("schema entry should be readable").path();
        if path.is_dir() {
            collect_case_files(&path, files);
        } else if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(".cases.json"))
        {
            files.push(path);
        }
    }
}

#[test]
fn validates_every_language_neutral_schema_case() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut case_files = Vec::new();
    collect_case_files(&root.join("schemas"), &mut case_files);
    case_files.sort();

    assert!(
        !case_files.is_empty(),
        "at least one schema case file is required"
    );

    for case_path in case_files {
        let cases: Value = serde_json::from_slice(
            &fs::read(&case_path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", case_path.display())),
        )
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", case_path.display()));
        let schema_reference = cases["schema"]
            .as_str()
            .unwrap_or_else(|| panic!("{} needs a schema string", case_path.display()));
        let schema_path = if schema_reference.starts_with("https://") {
            case_path.with_file_name("v1.schema.json")
        } else {
            case_path
                .parent()
                .expect("case file should have a parent")
                .join(schema_reference)
        };
        let schema: Value =
            serde_json::from_slice(&fs::read(&schema_path).unwrap_or_else(|error| {
                panic!("failed to read {}: {error}", schema_path.display())
            }))
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", schema_path.display()));
        let validator = jsonschema::draft202012::options()
            .should_validate_formats(true)
            .build(&schema)
            .unwrap_or_else(|error| panic!("failed to compile {}: {error}", schema_path.display()));

        let entries = cases["cases"]
            .as_array()
            .unwrap_or_else(|| panic!("{} needs a cases array", case_path.display()));
        let mut descriptions = std::collections::BTreeSet::new();
        for case in entries {
            let description = case["description"].as_str().unwrap_or_else(|| {
                panic!("{} has a case without a description", case_path.display())
            });
            assert!(
                descriptions.insert(description),
                "{} has duplicate case description {description:?}",
                case_path.display()
            );
            let expected = case["valid"]
                .as_bool()
                .unwrap_or_else(|| panic!("{description:?} needs a boolean valid expectation"));
            let actual = validator.is_valid(&case["instance"]);
            assert_eq!(
                actual,
                expected,
                "{} case failed: {description}",
                case_path.display()
            );
        }
    }
}
