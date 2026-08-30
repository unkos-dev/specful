//! Repository validation: walks an adopting repository, applies the profile
//! schemas and body checks, and enforces cross-artifact integrity.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::body::{ArtifactKind, check_body};
use crate::config::{Config, load_config};
use crate::diagnostics::{Finding, sort_findings};
use crate::frontmatter::split_frontmatter;
use crate::schemas::{
    ADR_V1_SCHEMA_ID, DESIGN_V1_SCHEMA_ID, REQUIREMENT_V1_SCHEMA_ID, builtin_schema,
};
use crate::yaml::load_restricted_yaml;

pub(crate) const ADR_DIR: &str = "docs/adr";
pub(crate) const SPECS_DIR: &str = "docs/specs";

/// One conformant artifact, as collected during the repository walk.
///
/// Only artifacts that pass frontmatter loading and schema validation are
/// collected; defective files contribute findings instead.
#[derive(Debug, Clone)]
pub(crate) struct Artifact {
    pub kind: ArtifactKind,
    pub id: String,
    pub path: String,
    pub title: String,
    pub status: Option<String>,
    pub supersedes: Vec<String>,
    pub superseded_by: Vec<String>,
    pub satisfies: Vec<String>,
    pub governed_by: Vec<String>,
}

/// Validates the repository rooted at `root` and returns sorted findings.
pub fn validate_repository(root: &Path) -> Vec<Finding> {
    let mut findings = Vec::new();
    let config = load_config(root, &mut findings);
    let (artifacts, _) = collect_artifacts(root, &mut findings);
    validate_integrity(config.as_ref(), &artifacts, &mut findings);
    crate::index::check_generated_views(root, &artifacts, &mut findings);
    sort_findings(&mut findings);
    findings
}

/// Walks the repository and returns every conformant artifact, appending
/// findings for each defect encountered on the way.
pub(crate) fn collect_artifacts(root: &Path, findings: &mut Vec<Finding>) -> (Vec<Artifact>, bool) {
    let mut artifacts = Vec::new();
    let mut complete = true;
    collect_adr_directory(root, &mut artifacts, findings, &mut complete);
    collect_specs_tree(root, &mut artifacts, findings, &mut complete);
    (artifacts, complete)
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
pub(crate) fn read_entries(
    root: &Path,
    dir: &Path,
    findings: &mut Vec<Finding>,
) -> Vec<(PathBuf, bool)> {
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

/// Rejects a top-level artifact root (`ADR_DIR` or `SPECS_DIR`) that is
/// itself a symlink, before it is ever opened: `read_entries` only catches a
/// symlink among a directory's entries, never the walk's own starting point.
fn reject_symlinked_root(root: &Path, dir: &Path, findings: &mut Vec<Finding>) -> bool {
    match std::fs::symlink_metadata(dir) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            findings.push(Finding::new(
                relative(root, dir),
                None,
                "symlink not allowed",
            ));
            false
        }
        _ => true,
    }
}

fn markdown_files(root: &Path, dir: &Path, findings: &mut Vec<Finding>) -> Vec<PathBuf> {
    read_entries(root, dir, findings)
        .into_iter()
        .filter(|(path, is_dir)| !is_dir && path.extension().is_some_and(|e| e == "md"))
        .map(|(path, _)| path)
        .collect()
}

fn filename_sequence(path: &str, file_name: &str, findings: &mut Vec<Finding>) -> Option<i64> {
    let sequence = file_name
        .strip_suffix(".md")
        .and_then(|stem| stem.split_at_checked(4))
        .filter(|(digits, slug)| {
            digits.chars().all(|c| c.is_ascii_digit())
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
        })
        .and_then(|(digits, _)| digits.parse::<i64>().ok())
        .filter(|sequence| *sequence >= 1);
    if sequence.is_none() {
        findings.push(Finding::new(
            path,
            None,
            "filename must be NNNN-short-slug.md with a lowercase slug of at most 64 characters",
        ));
    }
    sequence
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

fn check_scope_segments(path: &str, findings: &mut Vec<Finding>) {
    let segments: Vec<&str> = path.split('/').collect();
    let Some(scope_segments) = segments.get(2..segments.len().saturating_sub(2)) else {
        return;
    };
    for segment in scope_segments {
        let valid = !segment.is_empty()
            && segment.split('-').all(|part| {
                !part.is_empty()
                    && part
                        .chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            });
        if !valid {
            findings.push(Finding::new(
                path,
                None,
                format!("scope directory segment {segment:?} must be lowercase ASCII kebab-case"),
            ));
        }
    }
}

fn collect_adr_directory(
    root: &Path,
    artifacts: &mut Vec<Artifact>,
    findings: &mut Vec<Finding>,
    complete: &mut bool,
) {
    let validator = compile(ADR_V1_SCHEMA_ID);
    if !reject_symlinked_root(root, &root.join(ADR_DIR), findings) {
        *complete = false;
        return;
    }
    let finding_count = findings.len();
    let files = markdown_files(root, &root.join(ADR_DIR), findings);
    *complete &= findings.len() == finding_count;
    for file in files {
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
            *complete = false;
            continue;
        };
        if !apply_schema(&validator, &value, &path, findings) {
            *complete = false;
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
        artifacts.push(Artifact {
            kind: ArtifactKind::Adr,
            id,
            path,
            title: value["title"].as_str().unwrap_or_default().to_owned(),
            status: value["status"].as_str().map(str::to_owned),
            supersedes: string_array(&value, "supersedes"),
            superseded_by: string_array(&value, "superseded-by"),
            satisfies: Vec::new(),
            governed_by: Vec::new(),
        });
    }
}

fn collect_specs_tree(
    root: &Path,
    artifacts: &mut Vec<Artifact>,
    findings: &mut Vec<Finding>,
    complete: &mut bool,
) {
    let validators = ArtifactValidators {
        requirement: compile(REQUIREMENT_V1_SCHEMA_ID),
        design: compile(DESIGN_V1_SCHEMA_ID),
    };
    if !reject_symlinked_root(root, &root.join(SPECS_DIR), findings) {
        *complete = false;
        return;
    }
    let mut stack = vec![root.join(SPECS_DIR)];
    while let Some(dir) = stack.pop() {
        let finding_count = findings.len();
        let entries = read_entries(root, &dir, findings);
        *complete &= findings.len() == finding_count;
        for (file, is_dir) in entries {
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
            if file_name == "index.md" {
                continue;
            }
            *complete &=
                collect_artifact(root, &file, &file_name, artifacts, findings, &validators);
        }
    }
}

/// Compiled schema validators shared across every artifact collected from
/// the specs tree.
struct ArtifactValidators {
    requirement: jsonschema::Validator,
    design: jsonschema::Validator,
}

fn collect_artifact(
    root: &Path,
    file: &Path,
    file_name: &str,
    artifacts: &mut Vec<Artifact>,
    findings: &mut Vec<Finding>,
    validators: &ArtifactValidators,
) -> bool {
    let Some((path, value, body, body_first_line)) = load_artifact(root, file, findings) else {
        return false;
    };
    let artifact_type = value["type"].as_str().unwrap_or_default().to_owned();
    if artifact_type.is_empty() {
        findings.push(Finding::new(
            &path,
            Some(2),
            "artifact frontmatter requires a non-empty type",
        ));
        return false;
    }
    let (kind, validator, kind_dir) = match artifact_type.as_str() {
        "REQ" => (
            ArtifactKind::Requirement,
            &validators.requirement,
            "requirements",
        ),
        "DESIGN" => (ArtifactKind::Design, &validators.design, "design"),
        _ => {
            findings.push(Finding::new(
                &path,
                Some(2),
                format!("artifact type \"{artifact_type}\" is not REQ or DESIGN"),
            ));
            return false;
        }
    };

    let parent_dir = path.rsplit('/').nth(1).unwrap_or_default().to_owned();
    if parent_dir != kind_dir {
        findings.push(Finding::new(
            &path,
            None,
            format!("a {artifact_type} artifact belongs in a {kind_dir}/ directory"),
        ));
    }
    check_scope_segments(&path, findings);
    let sequence = filename_sequence(&path, file_name, findings);
    if !apply_schema(validator, &value, &path, findings) {
        return false;
    }
    let id = value["id"].as_str().expect("schema requires id").to_owned();
    check_id_matches_filename(&id, sequence, &path, findings);
    findings.extend(check_body(kind, &value, &body, &path, body_first_line));

    artifacts.push(Artifact {
        kind,
        id,
        path,
        title: value["title"].as_str().unwrap_or_default().to_owned(),
        status: None,
        supersedes: Vec::new(),
        superseded_by: Vec::new(),
        satisfies: string_array(&value, "satisfies"),
        governed_by: string_array(&value, "governed-by"),
    });
    true
}

fn validate_integrity(
    config: Option<&Config>,
    artifacts: &[Artifact],
    findings: &mut Vec<Finding>,
) {
    let mut artifact_paths: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for artifact in artifacts {
        artifact_paths
            .entry(&artifact.id)
            .or_default()
            .push(&artifact.path);
    }

    for (id, paths) in &artifact_paths {
        if paths.len() > 1 {
            findings.push(Finding::new(
                paths[1],
                None,
                format!(
                    "artifact identifier {id} is already declared in {}",
                    paths[0]
                ),
            ));
        }
    }

    if let Some(config) = config {
        let prefix = format!("{}-", config.project_key);
        for (id, paths) in &artifact_paths {
            if !id.starts_with(&prefix) {
                findings.push(Finding::new(
                    paths[0],
                    None,
                    format!(
                        "identifier {id} does not use the configured project key {}",
                        config.project_key
                    ),
                ));
            }
        }

        let mut max_allocated: BTreeMap<&str, i64> = BTreeMap::new();
        for id in artifact_paths.keys() {
            let mut segments = id.rsplit('-');
            let sequence: i64 = segments.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            let kind = segments.next().unwrap_or_default();
            for known in ["ADR", "REQ", "DESIGN"] {
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
                let field = match kind {
                    "ADR" => "adr",
                    "REQ" => "requirement",
                    "DESIGN" => "design",
                    other => other,
                };
                findings.push(Finding::new(
                    crate::config::CONFIG_FILE,
                    None,
                    format!("next-{field}-sequence {counter} lags allocated identifier sequence {highest}"),
                ));
            }
        }
    }

    let requirement_ids: BTreeSet<&str> = artifacts
        .iter()
        .filter(|a| a.kind == ArtifactKind::Requirement)
        .map(|a| a.id.as_str())
        .collect();
    let adrs: BTreeMap<&str, &Artifact> = artifacts
        .iter()
        .filter(|a| a.kind == ArtifactKind::Adr)
        .map(|a| (a.id.as_str(), a))
        .collect();

    for artifact in artifacts {
        for target in &artifact.satisfies {
            if !requirement_ids.contains(target.as_str()) {
                findings.push(Finding::new(
                    &artifact.path,
                    None,
                    format!("satisfies target {target} does not exist"),
                ));
            }
        }
        for target in &artifact.governed_by {
            if !adrs.contains_key(target.as_str()) {
                findings.push(Finding::new(
                    &artifact.path,
                    None,
                    format!("governed-by target {target} does not exist"),
                ));
            }
        }
    }

    validate_supersession(&adrs, findings);
}

/// Creates the directory components of `relative` under `root`, rejecting
/// any existing component that is a symlink ("symlink not allowed") or a
/// non-directory such as a plain file ("not a directory") — kept distinct
/// so the finding does not misreport an ordinary file obstruction as a
/// symlink escape. Missing components are created one at a time with
/// `create_dir`, never `create_dir_all` through an unverified path, so a
/// repository-controlled symlink cannot redirect the write outside the
/// root.
pub(crate) fn create_dir_verified(root: &Path, relative: &Path) -> Result<(), Finding> {
    let mut current = root.to_path_buf();
    for component in relative.components() {
        current.push(component);
        let relative_display = || {
            current
                .strip_prefix(root)
                .unwrap_or(&current)
                .components()
                .map(|c| c.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/")
        };
        match std::fs::symlink_metadata(&current) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() {
                    return Err(Finding::new(
                        relative_display(),
                        None,
                        "symlink not allowed",
                    ));
                }
                if !metadata.is_dir() {
                    return Err(Finding::new(relative_display(), None, "not a directory"));
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if let Err(create_error) = std::fs::create_dir(&current) {
                    return Err(Finding::new(
                        relative_display(),
                        None,
                        format!("cannot create directory: {create_error}"),
                    ));
                }
            }
            Err(error) => {
                return Err(Finding::new(
                    relative_display(),
                    None,
                    format!("cannot inspect directory: {error}"),
                ));
            }
        }
    }
    Ok(())
}

fn validate_supersession(adrs: &BTreeMap<&str, &Artifact>, findings: &mut Vec<Finding>) {
    for (id, record) in adrs {
        for target in &record.supersedes {
            if target == id {
                findings.push(Finding::new(
                    &record.path,
                    None,
                    "an ADR cannot supersede itself",
                ));
                continue;
            }
            match adrs.get(target.as_str()) {
                None => findings.push(Finding::new(
                    &record.path,
                    None,
                    format!("supersedes target {target} does not exist"),
                )),
                Some(other) if !other.superseded_by.iter().any(|s| s == id) => {
                    findings.push(Finding::new(
                        &record.path,
                        None,
                        format!(
                            "{id} supersedes {target}, but {target} does not record superseded-by {id}"
                        ),
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
            match adrs.get(target.as_str()) {
                None => findings.push(Finding::new(
                    &record.path,
                    None,
                    format!("superseded-by target {target} does not exist"),
                )),
                Some(other) if !other.supersedes.iter().any(|s| s == id) => {
                    findings.push(Finding::new(
                        &record.path,
                        None,
                        format!(
                            "{id} records superseded-by {target}, but {target} does not record supersedes {id}"
                        ),
                    ));
                }
                Some(_) => {}
            }
        }
        let status = record.status.as_deref().unwrap_or_default();
        if status == "superseded" && record.superseded_by.is_empty() {
            findings.push(Finding::new(
                &record.path,
                None,
                "a superseded ADR must record superseded-by",
            ));
        }
        if status != "superseded" && !record.superseded_by.is_empty() {
            findings.push(Finding::new(
                &record.path,
                None,
                "only a superseded ADR may record superseded-by",
            ));
        }
    }

    // Supersession must be acyclic; walk superseded-by edges from every node.
    for start in adrs.keys() {
        let mut current = *start;
        let mut seen = BTreeSet::from([*start]);
        while let Some(record) = adrs.get(current) {
            let Some(next) = record.superseded_by.first() else {
                break;
            };
            if !seen.insert(next) {
                findings.push(Finding::new(
                    &adrs[*start].path,
                    None,
                    format!("supersession cycle detected involving {start}"),
                ));
                break;
            }
            current = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn adr(id: &str, status: &str, supersedes: &[&str], superseded_by: &[&str]) -> Artifact {
        Artifact {
            kind: ArtifactKind::Adr,
            id: id.to_string(),
            path: format!("docs/adr/{}.md", id.to_lowercase()),
            title: "Example".to_string(),
            status: Some(status.to_string()),
            supersedes: supersedes.iter().map(|s| s.to_string()).collect(),
            superseded_by: superseded_by.iter().map(|s| s.to_string()).collect(),
            satisfies: Vec::new(),
            governed_by: Vec::new(),
        }
    }

    // The ADR schema's allOf already ties status to superseded-by presence
    // (superseded requires it, every other status forbids it), so no
    // schema-valid fixture can reach validate_supersession's own status
    // check. Exercise it directly against a hand-built adrs map instead.
    #[test]
    fn superseded_by_without_superseded_status_is_a_finding() {
        let one = adr("BAD-ADR-0001", "accepted", &[], &["BAD-ADR-0002"]);
        let two = adr("BAD-ADR-0002", "accepted", &["BAD-ADR-0001"], &[]);
        let adrs = BTreeMap::from([("BAD-ADR-0001", &one), ("BAD-ADR-0002", &two)]);

        let mut findings = Vec::new();
        validate_supersession(&adrs, &mut findings);

        assert!(
            findings
                .iter()
                .any(|f| f.message == "only a superseded ADR may record superseded-by"),
            "expected a status/superseded-by finding, got: {findings:?}"
        );
    }

    #[test]
    fn superseded_status_without_superseded_by_is_a_finding() {
        let one = adr("BAD-ADR-0001", "superseded", &[], &[]);
        let adrs = BTreeMap::from([("BAD-ADR-0001", &one)]);

        let mut findings = Vec::new();
        validate_supersession(&adrs, &mut findings);

        assert!(
            findings
                .iter()
                .any(|f| f.message == "a superseded ADR must record superseded-by"),
            "expected a status/superseded-by finding, got: {findings:?}"
        );
    }
}
