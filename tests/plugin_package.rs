// SPDX-License-Identifier: Apache-2.0

//! Package-shape checks for the `plugin/` tree against the vendored Agent Plugins 1.0 JSON
//! Schema and Agent Skills frontmatter validity for each shipped skill.
//!
//! The vendored schema at `tests/fixtures/plugin/agent-plugins-1.0.0.plugin.schema.json` is
//! `plugin.schema.json` from `agentplugins/agent-plugins-spec` commit
//! `ff8ab5e392cc87bd88d87c060815a87490e51003`, sha256
//! `0a4aad95ce337878ad38802ebf0daa3fde76abe3f65400c86bcbb1ec0b3ab883`.

use serde_json::{Value, json};
use specful::frontmatter::split_frontmatter;
use specful::yaml::load_restricted_yaml;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_json(path: &Path) -> Value {
    serde_json::from_str(
        &fs::read_to_string(path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display())),
    )
    .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()))
}

fn agent_plugins_schema() -> Value {
    read_json(&repo_root().join("tests/fixtures/plugin/agent-plugins-1.0.0.plugin.schema.json"))
}

fn agent_plugins_validator() -> jsonschema::Validator {
    jsonschema::draft202012::options()
        .build(&agent_plugins_schema())
        .expect("vendored Agent Plugins 1.0 schema compiles")
}

#[test]
fn root_manifest_accepts_against_agent_plugins_schema() {
    let validator = agent_plugins_validator();
    let manifest = read_json(&repo_root().join("plugin/plugin.json"));
    let errors: Vec<_> = validator.iter_errors(&manifest).collect();
    assert!(
        errors.is_empty(),
        "plugin/plugin.json should validate: {errors:?}"
    );
}

#[test]
fn root_manifest_carries_no_version_field() {
    let manifest = read_json(&repo_root().join("plugin/plugin.json"));
    assert!(
        manifest.get("version").is_none(),
        "plugin/plugin.json must not carry a version field"
    );
}

// The vendored Agent Plugins 1.0 schema itself permits a `version` field; the
// project's own contract does not. This exercises that stricter local policy
// against an inline fixture, distinct from the schema-conformance checks below.
#[test]
fn repository_policy_rejects_a_version_bearing_manifest() {
    const ALLOWED_KEYS: &[&str] = &[
        "$schema",
        "name",
        "description",
        "license",
        "homepage",
        "repository",
    ];
    let manifest = json!({
        "$schema": "https://agent-plugins.org/schemas/1.0.0/plugin.schema.json",
        "name": "specful",
        "version": "1.0.0",
    });
    let has_disallowed_key = manifest
        .as_object()
        .expect("manifest is an object")
        .keys()
        .any(|key| !ALLOWED_KEYS.contains(&key.as_str()));
    assert!(
        has_disallowed_key,
        "a version-bearing manifest must fail the project's allowed-key policy"
    );
    for key in read_json(&repo_root().join("plugin/plugin.json"))
        .as_object()
        .expect("manifest is an object")
        .keys()
    {
        assert!(
            ALLOWED_KEYS.contains(&key.as_str()),
            "plugin/plugin.json has a key {key:?} outside the project's allowed set"
        );
    }
}

#[test]
fn agent_plugins_schema_rejects_a_manifest_with_an_unknown_key() {
    let validator = agent_plugins_validator();
    let manifest = json!({
        "$schema": "https://agent-plugins.org/schemas/1.0.0/plugin.schema.json",
        "name": "specful",
        "unknownField": "not allowed",
    });
    assert!(!validator.is_valid(&manifest));
}

#[test]
fn agent_plugins_schema_rejects_a_manifest_missing_required_fields() {
    let validator = agent_plugins_validator();
    assert!(!validator.is_valid(&json!({"name": "specful"})));
    assert!(!validator.is_valid(
        &json!({"$schema": "https://agent-plugins.org/schemas/1.0.0/plugin.schema.json"})
    ));
}

#[test]
fn package_contains_no_harness_specific_manifests() {
    for path in [
        ".claude-plugin/marketplace.json",
        "plugin/.claude-plugin/plugin.json",
    ] {
        assert!(
            !repo_root().join(path).exists(),
            "harness-specific manifest must be absent: {path}"
        );
    }
}

fn skill_directories() -> Vec<PathBuf> {
    let skills_dir = repo_root().join("plugin/skills");
    let mut directories: Vec<_> = fs::read_dir(&skills_dir)
        .expect("plugin/skills should be readable")
        .map(|entry| entry.expect("skill entry should be readable").path())
        .filter(|path| path.is_dir())
        .collect();
    directories.sort();
    directories
}

fn is_valid_skill_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 64 {
        return false;
    }
    if name.starts_with('-') || name.ends_with('-') || name.contains("--") {
        return false;
    }
    name.bytes()
        .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

#[test]
fn every_skill_directory_has_the_ten_expected_skills() {
    let names: Vec<_> = skill_directories()
        .iter()
        .map(|path| {
            path.file_name()
                .expect("skill directory has a name")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(
        names,
        vec![
            "specful-adr",
            "specful-design",
            "specful-implement",
            "specful-index",
            "specful-plan",
            "specful-requirement",
            "specful-review",
            "specful-show",
            "specful-trace",
            "specful-validate",
        ]
    );
}

#[test]
fn plan_skill_ships_named_non_empty_regular_references() {
    let plan_references = repo_root().join("plugin/skills/specful-plan/references");
    let expected = ["arc-plan.md", "change-plan.md", "planning-craft.md"];

    let mut actual: Vec<_> = fs::read_dir(&plan_references)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", plan_references.display()))
        .map(|entry| entry.expect("plan reference entry should be readable"))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    actual.sort();
    assert_eq!(actual, expected);

    for name in expected {
        let path = plan_references.join(name);
        let metadata = fs::symlink_metadata(&path)
            .unwrap_or_else(|error| panic!("failed to inspect {}: {error}", path.display()));
        assert!(
            metadata.file_type().is_file(),
            "{}/{} must be a regular file",
            plan_references.display(),
            name
        );
        assert!(
            metadata.len() > 0,
            "{}/{} must not be empty",
            plan_references.display(),
            name
        );
    }
}

#[test]
fn every_skill_has_valid_agent_skills_frontmatter() {
    const PORTABLE_FIELDS: &[&str] = &[
        "name",
        "description",
        "license",
        "compatibility",
        "metadata",
        "allowed-tools",
    ];

    for directory in skill_directories() {
        let directory_name = directory
            .file_name()
            .expect("skill directory has a name")
            .to_string_lossy()
            .into_owned();
        let skill_path = directory.join("SKILL.md");
        let source = fs::read_to_string(&skill_path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", skill_path.display()));
        let path_label = skill_path.to_string_lossy();
        let split = split_frontmatter(&source, &path_label).unwrap_or_else(|findings| {
            panic!("{path_label} frontmatter split failed: {findings:?}")
        });
        let frontmatter = load_restricted_yaml(split.yaml, &path_label, split.yaml_first_line)
            .unwrap_or_else(|findings| {
                panic!("{path_label} frontmatter is not valid yaml: {findings:?}")
            });
        let fields = frontmatter
            .as_object()
            .unwrap_or_else(|| panic!("{path_label} frontmatter must be a mapping"));

        for key in fields.keys() {
            assert!(
                PORTABLE_FIELDS.contains(&key.as_str()),
                "{path_label} has a non-portable frontmatter field {key:?}"
            );
        }

        let name = fields
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("{path_label} needs a string name"));
        assert_eq!(
            name, directory_name,
            "{path_label} name must match its directory name"
        );
        assert!(
            is_valid_skill_name(name),
            "{path_label} name {name:?} violates the Agent Skills naming rules"
        );

        let description = fields
            .get("description")
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("{path_label} needs a non-empty string description"));
        assert!(
            !description.is_empty() && description.len() <= 1024,
            "{path_label} description must be 1 to 1024 characters"
        );

        if let Some(compatibility) = fields.get("compatibility") {
            let compatibility = compatibility
                .as_str()
                .unwrap_or_else(|| panic!("{path_label} compatibility must be a string"));
            assert!(
                !compatibility.is_empty() && compatibility.len() <= 500,
                "{path_label} compatibility must be 1 to 500 characters"
            );
        }

        assert!(
            !split.body.trim().is_empty(),
            "{path_label} must have a non-empty body after its frontmatter"
        );
    }
}

#[test]
fn review_skill_ships_named_non_empty_regular_references() {
    let review_references = repo_root().join("plugin/skills/specful-review/references");
    let expected = [
        "adr-review.md",
        "design-review.md",
        "report-format.md",
        "requirement-review.md",
    ];

    let mut actual: Vec<_> = fs::read_dir(&review_references)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", review_references.display()))
        .map(|entry| entry.expect("review reference entry should be readable"))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    actual.sort();
    assert_eq!(actual, expected);

    for name in expected {
        let path = review_references.join(name);
        let metadata = fs::symlink_metadata(&path)
            .unwrap_or_else(|error| panic!("failed to inspect {}: {error}", path.display()));
        assert!(
            metadata.file_type().is_file(),
            "{}/{} must be a regular file",
            review_references.display(),
            name
        );
        assert!(
            metadata.len() > 0,
            "{}/{} must not be empty",
            review_references.display(),
            name
        );
    }
}
