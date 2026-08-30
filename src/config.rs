//! Repository configuration loading.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::diagnostics::Finding;
use crate::schemas::{CONFIG_V1_SCHEMA_ID, builtin_schema};
use crate::yaml::load_restricted_yaml;

pub const CONFIG_FILE: &str = ".specful/config.yaml";

/// Searches `start` and its ancestors for the nearest directory containing
/// `.specful/config.yaml`, the root-selection rule command operations use
/// when no explicit root is given. Library operations always receive a root
/// explicitly and never call this.
pub fn discover_root(start: &Path) -> Result<PathBuf, Finding> {
    let mut current = start;
    loop {
        if current.join(CONFIG_FILE).is_file() {
            return Ok(current.to_path_buf());
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => {
                return Err(Finding::new(
                    CONFIG_FILE,
                    None,
                    "no .specful/config.yaml found in this directory or any ancestor",
                ));
            }
        }
    }
}

const COUNTER_KINDS: [(&str, &str); 3] = [
    ("next-adr-sequence", "ADR"),
    ("next-requirement-sequence", "REQ"),
    ("next-design-sequence", "DESIGN"),
];

#[derive(Debug, Clone)]
pub struct Config {
    pub project_key: String,
    pub specful_version: String,
    /// Allocation counters keyed by identifier kind (ADR, REQ, DESIGN).
    pub counters: BTreeMap<String, i64>,
}

impl Config {
    /// Canonical serialized form written by `init` and identifier
    /// allocation. Rewrites are wholesale: comments are not preserved.
    #[must_use]
    pub fn render(&self) -> String {
        format!(
            "config-version: 1\n\
             project-key: {}\n\
             specful-version: {}\n\
             next-adr-sequence: {}\n\
             next-requirement-sequence: {}\n\
             next-design-sequence: {}\n",
            self.project_key,
            self.specful_version,
            self.counters.get("ADR").copied().unwrap_or(1),
            self.counters.get("REQ").copied().unwrap_or(1),
            self.counters.get("DESIGN").copied().unwrap_or(1),
        )
    }
}

/// Loads and schema-validates `.specful/config.yaml` under `root`.
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
        specful_version: value["specful-version"]
            .as_str()
            .expect("schema guarantees a string")
            .to_owned(),
        counters,
    })
}
