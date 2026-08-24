//! Repository validation: walks an adopting repository, applies the profile
//! schemas and body checks, and enforces cross-artifact integrity.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::body::{ArtifactKind, check_body};
use crate::config::{Config, load_config};
use crate::diagnostics::{Finding, sort_findings};
use crate::frontmatter::split_frontmatter;
use crate::schemas::{ADR_V1_SCHEMA_ID, MSDD_V1_SCHEMA_ID, MSRS_V1_SCHEMA_ID, builtin_schema};
use crate::yaml::load_restricted_yaml;

const ADR_DIR: &str = "docs/adr";
const SPECS_DIR: &str = "docs/specs";

#[derive(Debug, Default)]
struct Inventory {
    /// Artifact id -> repository-relative path of its declaration.
    artifact_paths: BTreeMap<String, Vec<String>>,
    /// Requirement id -> declaring paths.
    requirement_paths: BTreeMap<String, Vec<String>>,
    /// (path, requirement ids referenced through satisfies).
    satisfies: Vec<(String, Vec<String>)>,
    /// (path, ADR ids referenced through governed-by).
    governed_by: Vec<(String, Vec<String>)>,
    /// ADR id -> (path, status, supersedes, superseded-by).
    adrs: BTreeMap<String, AdrRecord>,
}

#[derive(Debug, Clone)]
struct AdrRecord {
    path: String,
    status: String,
    supersedes: Vec<String>,
    superseded_by: Vec<String>,
}

/// Validates the repository rooted at `root` and returns sorted findings.
pub fn validate_repository(root: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();
    let config = load_config(root, &mut findings);
    let mut inventory = Inventory::default();

    validate_adr_directory(root, &mut inventory, &mut findings);
    validate_specs_tree(root, &mut inventory, &mut findings);
    validate_integrity(config.as_ref(), &inventory, &mut findings);

    sort_findings(&mut findings);
    findings
}

fn compile(schema_id: &str) -> jsonschema::Validator {
    let schema: serde_json::Value =
        serde_json::from_str(builtin_schema(schema_id).expect("schema is built in"))
            .expect("built-in schema is valid JSON");
    jsonschema::draft202012::options()
        .should_validate_formats(true)
        .build(&schema)
        .expect("built-in schema compiles")
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

/// Reads `dir`, reporting unreadable directories and rejecting symlinks.
///
/// A directory that does not exist is not a finding: a repository need not
/// carry every optional tree. Any other read failure is fail-closed, since
/// content that cannot be read cannot be validated.
fn read_entries(root: &Path, dir: &Path, findings: &mut Vec<Finding>) -> Vec<(PathBuf, bool)> {
    let mut kept = Vec::new();
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return kept,
        Err(error) => {
            findings.push(Finding::new(
                relative(root, dir),
                None,
                format!("cannot read directory: {error}"),
            ));
            return kept;
        }
    };
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                findings.push(Finding::new(
                    relative(root, dir),
                    None,
                    format!("cannot read directory entry: {error}"),
                ));
                continue;
            }
        };
        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(error) => {
                findings.push(Finding::new(
                    relative(root, &path),
                    None,
                    format!("cannot read directory entry: {error}"),
                ));
                continue;
            }
        };
        // `file_type` never follows symlinks, so a symlinked directory is
        // reported here instead of being walked into.
        if file_type.is_symlink() {
            findings.push(Finding::new(
                relative(root, &path),
                None,
                "symlink not allowed",
            ));
            continue;
        }
        kept.push((path, file_type.is_dir()));
    }
    kept.sort();
    kept
}

fn markdown_files(root: &Path, dir: &Path, findings: &mut Vec<Finding>) -> Vec<PathBuf> {
    read_entries(root, dir, findings)
        .into_iter()
        .filter(|(path, is_dir)| !is_dir && path.extension().is_some_and(|e| e == "md"))
        .map(|(path, _)| path)
        .collect()
}

fn filename_sequence(path: &str, file_name: &str, findings: &mut Vec<Finding>) -> Option<i64> {
    let stem = file_name.strip_suffix(".md")?;
    let (digits, slug) = stem.split_at_checked(4)?;
    if digits.chars().all(|c| c.is_ascii_digit())
        && slug.strip_prefix('-').is_some_and(|s| {
            !s.is_empty()
                && s.len() <= 64
                && s.split('-').all(|part| {
                    !part.is_empty()
                        && part
                            .chars()
                            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
                })
        })
    {
        let sequence: i64 = digits.parse().ok()?;
        if sequence >= 1 {
            return Some(sequence);
        }
    }
    findings.push(Finding::new(
        path,
        None,
        "filename must be NNNN-short-slug.md with a lowercase slug of at most 64 characters",
    ));
    None
}

fn id_sequence(id: &str) -> Option<i64> {
    id.rsplit('-').next()?.parse().ok()
}

fn string_array(value: &serde_json::Value, field: &str) -> Vec<String> {
    value[field]
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

fn load_artifact(
    root: &Path,
    file: &Path,
    findings: &mut Vec<Finding>,
) -> Option<(String, serde_json::Value, String, usize)> {
    let path = relative(root, file);
    let source = match std::fs::read_to_string(file) {
        Ok(source) => source,
        Err(error) => {
            findings.push(Finding::new(
                &path,
                None,
                format!("cannot read file: {error}"),
            ));
            return None;
        }
    };
    let split = match split_frontmatter(&source, &path) {
        Ok(split) => split,
        Err(mut errors) => {
            findings.append(&mut errors);
            return None;
        }
    };
    let value = match load_restricted_yaml(split.yaml, &path, split.yaml_first_line) {
        Ok(value) => value,
        Err(mut errors) => {
            findings.append(&mut errors);
            return None;
        }
    };
    if !value.is_object() {
        findings.push(Finding::new(
            &path,
            Some(2),
            "frontmatter must be a mapping",
        ));
        return None;
    }
    Some((path, value, split.body.to_owned(), split.body_first_line))
}

fn apply_schema(
    validator: &jsonschema::Validator,
    value: &serde_json::Value,
    path: &str,
    findings: &mut Vec<Finding>,
) -> bool {
    let mut conformant = true;
    for error in validator.iter_errors(value) {
        conformant = false;
        let pointer = error.instance_path().to_string();
        let location = if pointer.is_empty() {
            "frontmatter".to_owned()
        } else {
            format!("frontmatter {pointer}")
        };
        findings.push(Finding::new(path, None, format!("{location}: {error}")));
    }
    conformant
}

fn check_id_matches_filename(
    id: &str,
    sequence_from_name: Option<i64>,
    path: &str,
    findings: &mut Vec<Finding>,
) {
    if let (Some(file_seq), Some(id_seq)) = (sequence_from_name, id_sequence(id))
        && file_seq != id_seq
    {
        findings.push(Finding::new(
            path,
            None,
            format!("filename sequence {file_seq:04} does not match identifier {id}"),
        ));
    }
}

fn validate_adr_directory(root: &Path, inventory: &mut Inventory, findings: &mut Vec<Finding>) {
    let validator = compile(ADR_V1_SCHEMA_ID);
    for file in markdown_files(root, &root.join(ADR_DIR), findings) {
        let file_name = file
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if file_name == "README.md" {
            continue;
        }
        let path = relative(root, &file);
        let sequence = filename_sequence(&path, &file_name, findings);
        let Some((path, value, body, body_first_line)) = load_artifact(root, &file, findings)
        else {
            continue;
        };
        if !apply_schema(&validator, &value, &path, findings) {
            continue;
        }
        let id = value["id"].as_str().expect("schema requires id").to_owned();
        check_id_matches_filename(&id, sequence, &path, findings);
        findings.extend(check_body(
            ArtifactKind::Adr,
            &value,
            &body,
            &path,
            body_first_line,
        ));
        inventory
            .artifact_paths
            .entry(id.clone())
            .or_default()
            .push(path.clone());
        inventory.adrs.insert(
            id,
            AdrRecord {
                path,
                status: value["status"].as_str().unwrap_or_default().to_owned(),
                supersedes: string_array(&value, "supersedes"),
                superseded_by: string_array(&value, "superseded-by"),
            },
        );
    }
}

fn validate_specs_tree(root: &Path, inventory: &mut Inventory, findings: &mut Vec<Finding>) {
    let msrs_validator = compile(MSRS_V1_SCHEMA_ID);
    let msdd_validator = compile(MSDD_V1_SCHEMA_ID);
    let mut stack = vec![root.join(SPECS_DIR)];
    while let Some(dir) = stack.pop() {
        for (file, is_dir) in read_entries(root, &dir, findings) {
            if is_dir {
                stack.push(file);
                continue;
            }
            if !file.extension().is_some_and(|e| e == "md") {
                continue;
            }
            let file_name = file
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            if file_name == "index.md" || file_name == "log.md" {
                continue;
            }
            validate_concept(
                root,
                &file,
                &file_name,
                inventory,
                findings,
                &msrs_validator,
                &msdd_validator,
            );
        }
    }
}

fn validate_concept(
    root: &Path,
    file: &Path,
    file_name: &str,
    inventory: &mut Inventory,
    findings: &mut Vec<Finding>,
    msrs_validator: &jsonschema::Validator,
    msdd_validator: &jsonschema::Validator,
) {
    let Some((path, value, body, body_first_line)) = load_artifact(root, file, findings) else {
        return;
    };
    let concept_type = value["type"].as_str().unwrap_or_default().to_owned();
    if concept_type.is_empty() {
        findings.push(Finding::new(
            &path,
            Some(2),
            "concept frontmatter requires a non-empty type",
        ));
        return;
    }
    let (kind, validator, kind_dir) = match concept_type.as_str() {
        "MSRS" => (ArtifactKind::Msrs, msrs_validator, "msrs"),
        "MSDD" => (ArtifactKind::Msdd, msdd_validator, "msdd"),
        _ => return,
    };

    let parent_dir = path.rsplit('/').nth(1).unwrap_or_default().to_owned();
    if parent_dir != kind_dir {
        findings.push(Finding::new(
            &path,
            None,
            format!("a {concept_type} module belongs in an {kind_dir}/ directory"),
        ));
    }
    let sequence = filename_sequence(&path, file_name, findings);
    if !apply_schema(validator, &value, &path, findings) {
        return;
    }
    let id = value["id"].as_str().expect("schema requires id").to_owned();
    check_id_matches_filename(&id, sequence, &path, findings);
    findings.extend(check_body(kind, &value, &body, &path, body_first_line));
    inventory
        .artifact_paths
        .entry(id)
        .or_default()
        .push(path.clone());

    let governed = string_array(&value, "governed-by");
    if !governed.is_empty() {
        inventory.governed_by.push((path.clone(), governed));
    }
    match kind {
        ArtifactKind::Msrs => {
            if let Some(requirements) = value["requirements"].as_object() {
                for requirement_id in requirements.keys() {
                    inventory
                        .requirement_paths
                        .entry(requirement_id.clone())
                        .or_default()
                        .push(path.clone());
                }
            }
        }
        ArtifactKind::Msdd => {
            let satisfies = string_array(&value, "satisfies");
            if !satisfies.is_empty() {
                inventory.satisfies.push((path.clone(), satisfies));
            }
        }
        ArtifactKind::Adr => unreachable!("concepts are MSRS or MSDD"),
    }
}

fn validate_integrity(config: Option<&Config>, inventory: &Inventory, findings: &mut Vec<Finding>) {
    for (id, paths) in &inventory.artifact_paths {
        if paths.len() > 1 {
            findings.push(Finding::new(
                &paths[1],
                None,
                format!(
                    "artifact identifier {id} is already declared in {}",
                    paths[0]
                ),
            ));
        }
    }
    for (id, paths) in &inventory.requirement_paths {
        if paths.len() > 1 {
            findings.push(Finding::new(
                &paths[1],
                None,
                format!(
                    "requirement identifier {id} is already declared in {}",
                    paths[0]
                ),
            ));
        }
    }

    if let Some(config) = config {
        let prefix = format!("{}-", config.project_key);
        let all_ids = inventory
            .artifact_paths
            .keys()
            .map(|id| (id, &inventory.artifact_paths[id][0]))
            .chain(
                inventory
                    .requirement_paths
                    .keys()
                    .map(|id| (id, &inventory.requirement_paths[id][0])),
            );
        for (id, path) in all_ids {
            if !id.starts_with(&prefix) {
                findings.push(Finding::new(
                    path,
                    None,
                    format!(
                        "identifier {id} does not use the configured project key {}",
                        config.project_key
                    ),
                ));
            }
        }

        let mut max_allocated: BTreeMap<&str, i64> = BTreeMap::new();
        for id in inventory
            .artifact_paths
            .keys()
            .chain(inventory.requirement_paths.keys())
        {
            let mut segments = id.rsplit('-');
            let sequence: i64 = segments.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            let kind = segments.next().unwrap_or_default();
            for known in ["ADR", "MSRS", "REQ", "MSDD"] {
                if kind == known {
                    let entry = max_allocated.entry(known).or_insert(0);
                    *entry = (*entry).max(sequence);
                }
            }
        }
        for (kind, highest) in max_allocated {
            if let Some(counter) = config.counters.get(kind)
                && *counter <= highest
            {
                findings.push(Finding::new(
                    crate::config::CONFIG_FILE,
                    None,
                    format!(
                        "next-{}-sequence {counter} lags allocated identifier sequence {highest}",
                        kind.to_ascii_lowercase().replace("req", "requirement"),
                    ),
                ));
            }
        }
    }

    for (path, targets) in &inventory.satisfies {
        for target in targets {
            if !inventory.requirement_paths.contains_key(target) {
                findings.push(Finding::new(
                    path,
                    None,
                    format!("satisfies target {target} does not exist"),
                ));
            }
        }
    }
    for (path, targets) in &inventory.governed_by {
        for target in targets {
            if !inventory.adrs.contains_key(target) {
                findings.push(Finding::new(
                    path,
                    None,
                    format!("governed-by target {target} does not exist"),
                ));
            }
        }
    }

    validate_supersession(inventory, findings);
}

fn validate_supersession(inventory: &Inventory, findings: &mut Vec<Finding>) {
    for (id, record) in &inventory.adrs {
        for target in &record.supersedes {
            if target == id {
                findings.push(Finding::new(
                    &record.path,
                    None,
                    "an ADR cannot supersede itself",
                ));
                continue;
            }
            match inventory.adrs.get(target) {
                None => findings.push(Finding::new(
                    &record.path,
                    None,
                    format!("supersedes target {target} does not exist"),
                )),
                Some(other) if !other.superseded_by.contains(id) => {
                    findings.push(Finding::new(
                        &record.path,
                        None,
                        format!("{id} supersedes {target}, but {target} does not record superseded-by {id}"),
                    ));
                }
                Some(_) => {}
            }
        }
        for target in &record.superseded_by {
            if target == id {
                findings.push(Finding::new(
                    &record.path,
                    None,
                    "an ADR cannot be superseded by itself",
                ));
                continue;
            }
            match inventory.adrs.get(target) {
                None => findings.push(Finding::new(
                    &record.path,
                    None,
                    format!("superseded-by target {target} does not exist"),
                )),
                Some(other) if !other.supersedes.contains(id) => {
                    findings.push(Finding::new(
                        &record.path,
                        None,
                        format!("{id} records superseded-by {target}, but {target} does not record supersedes {id}"),
                    ));
                }
                Some(_) => {}
            }
        }
        if record.status == "superseded" && record.superseded_by.is_empty() {
            findings.push(Finding::new(
                &record.path,
                None,
                "a superseded ADR must record superseded-by",
            ));
        }
        if record.status != "superseded" && !record.superseded_by.is_empty() {
            findings.push(Finding::new(
                &record.path,
                None,
                "only a superseded ADR may record superseded-by",
            ));
        }
    }

    // Supersession must be acyclic; walk supersedes edges from every node.
    for start in inventory.adrs.keys() {
        let mut current = start;
        let mut seen = BTreeSet::from([start]);
        while let Some(record) = inventory.adrs.get(current) {
            let Some(next) = record.superseded_by.first() else {
                break;
            };
            if !seen.insert(next) {
                findings.push(Finding::new(
                    &inventory.adrs[start].path,
                    None,
                    format!("supersession cycle detected involving {start}"),
                ));
                break;
            }
            current = next;
        }
    }
}
