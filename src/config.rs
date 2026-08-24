//! Repository configuration loading.

use std::collections::BTreeMap;
use std::path::Path;

use crate::diagnostics::Finding;
use crate::schemas::{CONFIG_V1_SCHEMA_ID, builtin_schema};
use crate::yaml::load_restricted_yaml;

pub const CONFIG_FILE: &str = ".specful.yaml";

const COUNTER_KINDS: [(&str, &str); 4] = [
    ("next-adr-sequence", "ADR"),
    ("next-msrs-sequence", "MSRS"),
    ("next-requirement-sequence", "REQ"),
    ("next-msdd-sequence", "MSDD"),
];

#[derive(Debug, Clone)]
pub struct Config {
    pub project_key: String,
    /// Allocation counters keyed by identifier kind (ADR, MSRS, REQ, MSDD).
    pub counters: BTreeMap<String, i64>,
}

/// Loads and schema-validates `.specful.yaml` under `root`.
///
/// Findings are appended for every problem; `None` means the configuration
/// is unusable and dependent checks should be skipped.
pub fn load_config(root: &Path, findings: &mut Vec<Finding>) -> Option<Config> {
    let path = root.join(CONFIG_FILE);
    let source = match std::fs::read_to_string(&path) {
        Ok(source) => source,
        Err(error) => {
            findings.push(Finding::new(
                CONFIG_FILE,
                None,
                format!("cannot read repository configuration: {error}"),
            ));
            return None;
        }
    };

    let value = match load_restricted_yaml(&source, CONFIG_FILE, 1) {
        Ok(value) => value,
        Err(mut errors) => {
            findings.append(&mut errors);
            return None;
        }
    };

    let schema: serde_json::Value = serde_json::from_str(
        builtin_schema(CONFIG_V1_SCHEMA_ID).expect("config schema is built in"),
    )
    .expect("built-in config schema is valid JSON");
    let validator = jsonschema::draft202012::options()
        .should_validate_formats(true)
        .build(&schema)
        .expect("built-in config schema compiles");

    let mut conformant = true;
    for error in validator.iter_errors(&value) {
        conformant = false;
        findings.push(Finding::new(
            CONFIG_FILE,
            None,
            format!(
                "configuration does not conform at {}: {}",
                error.instance_path(),
                error
            ),
        ));
    }
    if !conformant {
        return None;
    }

    let mut counters = BTreeMap::new();
    for (field, kind) in COUNTER_KINDS {
        counters.insert(
            kind.to_owned(),
            value[field].as_i64().expect("schema guarantees an integer"),
        );
    }

    Some(Config {
        project_key: value["project-key"]
            .as_str()
            .expect("schema guarantees a string")
            .to_owned(),
        counters,
    })
}
