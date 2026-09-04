//! Repository scaffolding and artifact creation.
//!
//! `init` writes the canonical configuration and directory skeleton. `new`
//! allocates the next stable identifier and instantiates a schema-conformant
//! scaffold whose remaining `{...}` placeholders are the author's task;
//! `validate` reports them until the document is completed.

use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use crate::config::{CONFIG_FILE, Config, load_config};
use crate::diagnostics::Finding;
use crate::repo::{ADR_DIR, SPECS_DIR, create_dir_verified};

/// Sibling lock file guarding the read-modify-rename of `.specful/config.yaml`.
/// Its presence is the allocation lock: a second `specful new` sees
/// `create_new` fail and reports the collision instead of racing the first.
const LOCK_FILE: &str = ".specful/config.yaml.lock";

/// Root-discovery sentinel directory holding both canonical configuration
/// and generated views.
const CONFIG_DIR: &str = ".specful";

/// Scaffold sources: the only copies of artifact shape, so the CLI and the
/// public templates cannot drift.
const ADR_TEMPLATE: &str = include_str!("../templates/adr.md");
const REQUIREMENT_TEMPLATE: &str = include_str!("../templates/requirement.md");
const DESIGN_TEMPLATE: &str = include_str!("../templates/design.md");

/// Removes files this `init` invocation exclusively created, once a later
/// write fails.
fn rollback(root: &Path, files: &[&str]) {
    for file in files {
        let _ = std::fs::remove_file(root.join(file));
    }
}

fn validate_project_key(project_key: &str) -> Result<(), Vec<Finding>> {
    if !project_key
        .strip_prefix(|c: char| c.is_ascii_uppercase())
        .is_some_and(|rest| {
            (1..=9).contains(&rest.len())
                && rest
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        })
    {
        return Err(vec![Finding::new(
            CONFIG_FILE,
            None,
            "project key must be 2 to 10 uppercase letters or digits, starting with a letter",
        )]);
    }
    Ok(())
}

/// Owns the lock file created for one configuration update. Dropping the
/// guard without calling [`ConfigLock::commit`] successfully removes the
/// lock file, so every error path after acquisition releases it.
struct ConfigLock {
    path: PathBuf,
    committed: bool,
}

impl ConfigLock {
    /// Acquires the lock by exclusively creating the lock file. An existing
    /// lock means another process may be running, or a stale lock remains
    /// from an interrupted run and must be removed manually; either way this
    /// never removes it itself.
    fn acquire(root: &Path) -> Result<Self, Vec<Finding>> {
        let path = root.join(LOCK_FILE);
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(_) => Ok(Self {
                path,
                committed: false,
            }),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => Err(vec![Finding::new(
                LOCK_FILE,
                None,
                "lock file already exists; another specful process may be running, or a stale lock remains and must be removed manually",
            )]),
            Err(error) => Err(vec![Finding::new(
                LOCK_FILE,
                None,
                format!("cannot create lock file: {error}"),
            )]),
        }
    }

    /// Writes `content` to the lock file, then renames it onto
    /// `.specful/config.yaml`: an atomic replace that also releases the lock. On
    /// any error the lock file is left for `Drop` to remove.
    fn commit(mut self, root: &Path, content: &str) -> Result<(), Vec<Finding>> {
        if let Err(error) = std::fs::write(&self.path, content) {
            return Err(vec![Finding::new(
                CONFIG_FILE,
                None,
                format!("cannot write configuration: {error}"),
            )]);
        }
        if let Err(error) = std::fs::rename(&self.path, root.join(CONFIG_FILE)) {
            return Err(vec![Finding::new(
                CONFIG_FILE,
                None,
                format!("cannot commit configuration: {error}"),
            )]);
        }
        self.committed = true;
        Ok(())
    }
}

impl Drop for ConfigLock {
    fn drop(&mut self) {
        if !self.committed {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewKind {
    Adr,
    Requirement,
    Design,
}

/// Result of a successful [`init`]: paths created fresh.
#[derive(Debug, Clone, Default)]
pub struct InitOutcome {
    pub created: Vec<String>,
}

/// Initializes a Specful repository with configuration and the artifact
/// directory skeleton. Refuses to touch an already-initialized root.
///
/// Preconditions run before any write, in order: project key,
/// `.specful/config.yaml` absence (an early advisory check purely for error
/// ordering; `create_new` below is the actual enforcement and the
/// concurrency lock). A later write failure rolls back every file this
/// invocation exclusively created. If
/// configuration creation fails after the directories were created (for
/// example a concurrent or prior `init`), those directories are left in
/// place: they are empty and harmless, and a rerun completes the job
/// through `create_dir_verified`'s existing-directory tolerance.
pub fn init(root: &Path, project_key: &str) -> Result<InitOutcome, Vec<Finding>> {
    validate_project_key(project_key)?;

    if root.join(CONFIG_FILE).exists() {
        return Err(vec![Finding::new(
            CONFIG_FILE,
            None,
            "repository is already initialized",
        )]);
    }
    let config = Config {
        project_key: project_key.to_owned(),
        counters: BTreeMap::from([
            ("ADR".to_owned(), 1),
            ("REQ".to_owned(), 1),
            ("DESIGN".to_owned(), 1),
        ]),
    };

    let mut created = Vec::new();
    for dir in [ADR_DIR, SPECS_DIR] {
        create_dir_verified(root, Path::new(dir)).map_err(|finding| vec![finding])?;
        created.push(format!("{dir}/"));
    }
    create_dir_verified(root, Path::new(CONFIG_DIR)).map_err(|finding| vec![finding])?;

    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join(CONFIG_FILE))
    {
        Ok(mut file) => {
            if let Err(error) = file.write_all(config.render().as_bytes()) {
                // create_new just claimed this path exclusively; leaving a
                // truncated .specful/config.yaml behind would make a rerun of
                // init see "repository is already initialized" against a
                // corrupt config, so remove it rather than leave the
                // partial write in place.
                drop(file);
                let _ = std::fs::remove_file(root.join(CONFIG_FILE));
                return Err(vec![Finding::new(
                    CONFIG_FILE,
                    None,
                    format!("cannot write configuration: {error}"),
                )]);
            }
        }
        Err(error) if error.kind() == ErrorKind::AlreadyExists => {
            return Err(vec![Finding::new(
                CONFIG_FILE,
                None,
                "repository is already initialized",
            )]);
        }
        Err(error) => {
            return Err(vec![Finding::new(
                CONFIG_FILE,
                None,
                format!("cannot write configuration: {error}"),
            )]);
        }
    }
    created.push(CONFIG_FILE.to_owned());

    let generated_views = crate::index::render_views(&[]);
    // A view path that already exists is not this invocation's to remove:
    // run_index refuses author-owned files, and rollback must not delete
    // the very file that refusal protected.
    let preexisting_views: Vec<&String> = generated_views
        .keys()
        .filter(|path| root.join(path).exists())
        .collect();
    let view_findings = crate::index::run_index(root, false);
    if !view_findings.is_empty() {
        let mut rollback_targets = vec![CONFIG_FILE];
        rollback_targets.extend(
            generated_views
                .keys()
                .filter(|path| !preexisting_views.contains(path))
                .map(String::as_str),
        );
        rollback(root, &rollback_targets);
        return Err(view_findings);
    }
    created.extend(generated_views.into_keys());

    Ok(InitOutcome { created })
}

/// Creates one artifact from its scaffold, allocating identifiers from the
/// repository configuration. Returns the repository-relative path written.
pub fn new_artifact(
    root: &Path,
    kind: NewKind,
    scope: Option<&str>,
    title: &str,
) -> Result<String, Vec<Finding>> {
    let scope = match (kind, scope) {
        (NewKind::Adr, Some(_)) => {
            return Err(vec![Finding::new(
                ADR_DIR,
                None,
                "an ADR takes no scope; the ADR directory is flat",
            )]);
        }
        (NewKind::Adr, None) => None,
        (NewKind::Requirement | NewKind::Design, None) => {
            return Err(vec![Finding::new(
                SPECS_DIR,
                None,
                "a requirement or design needs --scope, an architectural path such as backend or system/sync",
            )]);
        }
        (NewKind::Requirement | NewKind::Design, Some(scope)) => {
            let well_formed = !scope.is_empty()
                && scope.split('/').all(|segment| {
                    !segment.is_empty()
                        && segment
                            .chars()
                            .next()
                            .is_some_and(|c| c.is_ascii_lowercase())
                        && segment
                            .chars()
                            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
                });
            if !well_formed {
                return Err(vec![Finding::new(
                    SPECS_DIR,
                    None,
                    "scope segments use lowercase letters, digits, and hyphens",
                )]);
            }
            Some(scope)
        }
    };

    let slug = slugify(title);
    if slug.is_empty() {
        return Err(vec![Finding::new(
            CONFIG_FILE,
            None,
            "title yields an empty filename slug; use letters or digits",
        )]);
    }

    // The lock file is the exclusive right to read, allocate from, and
    // rewrite `.specful/config.yaml`, so the whole read-modify-write below is one
    // critical section: no other allocation can observe or advance these
    // counters between the read and the rename that commits them. It is
    // released (via `Drop`) on every path out of this function once
    // acquired, whether by the atomic rename on success or by an error
    // return before it.
    let lock = ConfigLock::acquire(root)?;

    let mut findings = Vec::new();
    let Some(mut config) = load_config(root, &mut findings) else {
        return Err(findings);
    };

    let counter_kind = match kind {
        NewKind::Adr => "ADR",
        NewKind::Requirement => "REQ",
        NewKind::Design => "DESIGN",
    };
    let sequence = allocate(&mut config, counter_kind)?;
    let id = format!("{}-{counter_kind}-{sequence:04}", config.project_key);

    let (path, content) = match kind {
        NewKind::Adr => (
            format!("{ADR_DIR}/{sequence:04}-{slug}.md"),
            adr_scaffold(&id, title),
        ),
        NewKind::Requirement => (
            format!(
                "{SPECS_DIR}/{}/requirements/{sequence:04}-{slug}.md",
                scope.unwrap()
            ),
            requirement_scaffold(&id, title),
        ),
        NewKind::Design => (
            format!(
                "{SPECS_DIR}/{}/design/{sequence:04}-{slug}.md",
                scope.unwrap()
            ),
            design_scaffold(&id, title),
        ),
    };

    let target = root.join(&path);

    // Persist the advanced counters before the artifact: an interrupted run
    // then leaves an allocation gap, which is permitted, rather than a
    // counter that lags an allocated identifier, which is invalid.
    lock.commit(root, &config.render())?;

    if let Some(parent) = Path::new(&path).parent() {
        create_dir_verified(root, parent).map_err(|finding| vec![finding])?;
    }
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&target)
    {
        Ok(mut file) => {
            if let Err(error) = file.write_all(content.as_bytes()) {
                // `create_new` just claimed this path exclusively; leaving a
                // truncated file behind on a write failure would strand an
                // invalid artifact under an already-advanced counter, so
                // remove it rather than leave the partial write in place. A
                // hard kill mid-write can still leave a partial file; that
                // gap is the same accepted class as a skipped identifier.
                drop(file);
                let _ = std::fs::remove_file(&target);
                return Err(vec![Finding::new(
                    &path,
                    None,
                    format!("cannot write artifact: {error}"),
                )]);
            }
        }
        Err(error) if error.kind() == ErrorKind::AlreadyExists => {
            return Err(vec![Finding::new(&path, None, "file already exists")]);
        }
        Err(error) => {
            return Err(vec![Finding::new(
                &path,
                None,
                format!("cannot write artifact: {error}"),
            )]);
        }
    }
    Ok(path)
}

fn allocate(config: &mut Config, kind: &str) -> Result<i64, Vec<Finding>> {
    let counter = config
        .counters
        .get_mut(kind)
        .expect("configuration schema defines every counter");
    if *counter >= 10000 {
        return Err(vec![Finding::new(
            CONFIG_FILE,
            None,
            format!("the {kind} identifier sequence is exhausted"),
        )]);
    }
    let allocated = *counter;
    *counter += 1;
    Ok(allocated)
}

fn slugify(title: &str) -> String {
    let mut slug = String::new();
    for c in title.chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let trimmed = slug.trim_matches('-');
    trimmed
        .chars()
        .take(64)
        .collect::<String>()
        .trim_matches('-')
        .to_owned()
}

/// Today's date as `YYYY-MM-DD` in UTC.
fn today() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock is after 1970")
        .as_secs();
    let (year, month, day) = civil_from_days((seconds / 86_400) as i64);
    format!("{year:04}-{month:02}-{day:02}")
}

/// Howard Hinnant's civil-from-days algorithm; `z` is days since the epoch.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (if month <= 2 { year + 1 } else { year }, month, day)
}

/// YAML double-quoted scalar for arbitrary title text.
fn quote(text: &str) -> String {
    serde_json::to_string(text).expect("strings serialize")
}

/// Replaces, line by line, the first occurrence of each given field prefix
/// with the same prefix followed by its substitute value; every other line
/// of `template` passes through unchanged. Every template scaffold ends with
/// a single trailing newline, so reassembling `lines()` with an appended `\n`
/// per line reproduces it exactly.
fn substitute_lines(template: &str, replacements: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(template.len());
    let mut done = vec![false; replacements.len()];
    for line in template.lines() {
        let mut wrote = false;
        for (index, (prefix, value)) in replacements.iter().enumerate() {
            if !done[index] && line.starts_with(prefix) {
                out.push_str(prefix);
                out.push_str(value);
                out.push('\n');
                done[index] = true;
                wrote = true;
                break;
            }
        }
        if !wrote {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn adr_scaffold(id: &str, title: &str) -> String {
    substitute_lines(
        ADR_TEMPLATE,
        &[
            ("id: \"", &format!("{}\"", quote_inner(id))),
            ("title: \"", &format!("{}\"", quote_inner(title))),
            ("recorded-on: \"", &format!("{}\"", today())),
            ("# ", title),
        ],
    )
}

fn requirement_scaffold(id: &str, title: &str) -> String {
    substitute_lines(
        REQUIREMENT_TEMPLATE,
        &[
            ("id: \"", &format!("{}\"", quote_inner(id))),
            ("title: \"", &format!("{}\"", quote_inner(title))),
            ("# ", title),
        ],
    )
}

fn design_scaffold(id: &str, title: &str) -> String {
    substitute_lines(
        DESIGN_TEMPLATE,
        &[
            ("id: \"", &format!("{}\"", quote_inner(id))),
            ("title: \"", &format!("{}\"", quote_inner(title))),
            ("# ", title),
        ],
    )
}

/// The double-quoted scalar's inner content, without the surrounding quotes,
/// for splicing into a `prefix: "..."` line already carrying the opening
/// quote.
fn quote_inner(text: &str) -> String {
    let quoted = quote(text);
    quoted[1..quoted.len() - 1].to_owned()
}

#[cfg(test)]
mod tests {
    use super::{civil_from_days, slugify};

    #[test]
    fn converts_epoch_days_to_civil_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(19_723), (2024, 1, 1));
        assert_eq!(civil_from_days(20_679), (2026, 8, 14));
    }

    #[test]
    fn slugs_are_lowercase_bounded_and_hyphenated() {
        assert_eq!(
            slugify("Progress sync requirements"),
            "progress-sync-requirements"
        );
        assert_eq!(slugify("  If-Match, revisited!  "), "if-match-revisited");
        assert_eq!(slugify("???"), "");
        assert!(slugify(&"long word ".repeat(20)).len() <= 64);
    }
}
