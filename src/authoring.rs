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

const AGENTS_FILE: &str = "AGENTS.md";
const SPECFUL_MD_FILE: &str = "docs/SPECFUL.md";

/// Marker pair delimiting the managed block `init` installs in `AGENTS.md`.
/// Shared by the writer and the upsert so a well-formed file always has
/// exactly one of each, in this order.
pub const SPECFUL_MARKER_START: &str = "<!-- SPECFUL:START -->";
pub const SPECFUL_MARKER_END: &str = "<!-- SPECFUL:END -->";

const AGENTS_BLOCK_BODY: &str = include_str!("../templates/agents-block.md");
const SPECFUL_MD_CONTENT: &str = include_str!("../templates/SPECFUL.md");

/// Scaffold sources: the only copies of artifact shape, so the CLI and the
/// public templates cannot drift.
const ADR_TEMPLATE: &str = include_str!("../templates/adr.md");
const REQUIREMENT_TEMPLATE: &str = include_str!("../templates/requirement.md");
const DESIGN_TEMPLATE: &str = include_str!("../templates/design.md");

/// The documentation link must match the installed CLI, not a moving
/// branch, so the release tag is substituted at write time.
fn specful_md_content() -> String {
    SPECFUL_MD_CONTENT.replace("{version}", concat!("v", env!("CARGO_PKG_VERSION")))
}

/// The marker span plus its body, without a trailing newline: the unit
/// spliced into an existing well-formed span.
fn block_span() -> String {
    format!(
        "{SPECFUL_MARKER_START}\n{}\n{SPECFUL_MARKER_END}",
        AGENTS_BLOCK_BODY.trim_end()
    )
}

/// The block as a standalone file body: used both for a fresh `AGENTS.md`
/// and as the chunk appended after an existing file's content.
fn wrapped_block() -> String {
    format!("{}\n", block_span())
}

/// Where the marker pair sits in an `AGENTS.md` body, if at all.
enum MarkerState {
    /// Neither marker string occurs; the block can be appended.
    Absent,
    /// Exactly one of each, START before END. `end` is the byte offset just
    /// past the END marker, so `content[start..end]` is the whole span.
    WellFormed { start: usize, end: usize },
    /// Any other arrangement: missing partner, reversed order, or
    /// duplicates. Never written to; the caller reports a finding.
    Malformed,
}

fn marker_state(content: &str) -> MarkerState {
    let starts: Vec<usize> = content
        .match_indices(SPECFUL_MARKER_START)
        .map(|(index, _)| index)
        .collect();
    let ends: Vec<usize> = content
        .match_indices(SPECFUL_MARKER_END)
        .map(|(index, _)| index)
        .collect();
    match (starts.as_slice(), ends.as_slice()) {
        ([], []) => MarkerState::Absent,
        (&[start], &[end]) if start < end => MarkerState::WellFormed {
            start,
            end: end + SPECFUL_MARKER_END.len(),
        },
        _ => MarkerState::Malformed,
    }
}

/// What `init` must do to `AGENTS.md`, resolved during the precondition
/// pass so the write pass never needs to re-inspect or re-read the file.
enum AgentsPlan {
    /// No `AGENTS.md` exists yet; write one containing only the block.
    Create,
    /// The full replacement content for an existing `AGENTS.md`: the
    /// block appended after a blank line, or spliced between well-formed
    /// markers.
    Upsert(String),
}

/// Precondition check for `AGENTS.md`: inspected without following
/// symlinks, must be a regular file if present, must be readable, and its
/// marker arrangement (if any) must be well-formed. Resolves the upsert
/// content here so the write stage is a single atomic rename.
fn plan_agents_md(path: &Path) -> Result<AgentsPlan, Vec<Finding>> {
    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(AgentsPlan::Create),
        Err(error) => {
            return Err(vec![Finding::new(
                AGENTS_FILE,
                None,
                format!("cannot inspect {AGENTS_FILE}: {error}"),
            )]);
        }
    };
    if metadata.file_type().is_symlink() {
        return Err(vec![Finding::new(AGENTS_FILE, None, "symlink not allowed")]);
    }
    if !metadata.is_file() {
        return Err(vec![Finding::new(AGENTS_FILE, None, "not a regular file")]);
    }
    let content = std::fs::read_to_string(path).map_err(|error| {
        vec![Finding::new(
            AGENTS_FILE,
            None,
            format!("cannot read {AGENTS_FILE}: {error}"),
        )]
    })?;
    match marker_state(&content) {
        MarkerState::Absent => {
            let mut next = content;
            if !next.ends_with('\n') {
                next.push('\n');
            }
            next.push('\n');
            next.push_str(&wrapped_block());
            Ok(AgentsPlan::Upsert(next))
        }
        MarkerState::WellFormed { start, end } => {
            let mut next = String::with_capacity(content.len());
            next.push_str(&content[..start]);
            next.push_str(&block_span());
            next.push_str(&content[end..]);
            Ok(AgentsPlan::Upsert(next))
        }
        MarkerState::Malformed => Err(vec![Finding::new(
            AGENTS_FILE,
            None,
            "markers are malformed: a well-formed file has exactly one START and one END marker, START first",
        )]),
    }
}

/// Writes `docs/SPECFUL.md`, the specful-owned instruction file. `docs/`
/// already exists by this point: the directory scaffold created it.
fn write_specful_md(root: &Path) -> Result<(), Vec<Finding>> {
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join(SPECFUL_MD_FILE))
    {
        Ok(mut file) => {
            if let Err(error) = file.write_all(specful_md_content().as_bytes()) {
                drop(file);
                let _ = std::fs::remove_file(root.join(SPECFUL_MD_FILE));
                return Err(vec![Finding::new(
                    SPECFUL_MD_FILE,
                    None,
                    format!("cannot write {SPECFUL_MD_FILE}: {error}"),
                )]);
            }
            Ok(())
        }
        Err(error) if error.kind() == ErrorKind::AlreadyExists => Err(vec![Finding::new(
            SPECFUL_MD_FILE,
            None,
            "file already exists",
        )]),
        Err(error) => Err(vec![Finding::new(
            SPECFUL_MD_FILE,
            None,
            format!("cannot write {SPECFUL_MD_FILE}: {error}"),
        )]),
    }
}

/// Creates `AGENTS.md` containing only the marker block.
fn create_agents_md(root: &Path) -> Result<(), Vec<Finding>> {
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(root.join(AGENTS_FILE))
    {
        Ok(mut file) => {
            if let Err(error) = file.write_all(wrapped_block().as_bytes()) {
                drop(file);
                let _ = std::fs::remove_file(root.join(AGENTS_FILE));
                return Err(vec![Finding::new(
                    AGENTS_FILE,
                    None,
                    format!("cannot write {AGENTS_FILE}: {error}"),
                )]);
            }
            Ok(())
        }
        Err(error) if error.kind() == ErrorKind::AlreadyExists => {
            Err(vec![Finding::new(AGENTS_FILE, None, "file already exists")])
        }
        Err(error) => Err(vec![Finding::new(
            AGENTS_FILE,
            None,
            format!("cannot write {AGENTS_FILE}: {error}"),
        )]),
    }
}

/// Stages `content` in a sibling temporary file, then renames it onto
/// `target`: an interrupted run leaves either the old or the new content,
/// never a truncation. Mirrors `ConfigLock::commit` without needing a
/// separate lock file, since `.specful/config.yaml` already gates concurrent
/// `init` calls before this ever runs.
fn atomic_replace(target: &Path, content: &str) -> Result<(), Vec<Finding>> {
    let tmp_path = target.with_extension("md.tmp");
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp_path)
    {
        Ok(mut file) => {
            if let Err(error) = file.write_all(content.as_bytes()) {
                drop(file);
                let _ = std::fs::remove_file(&tmp_path);
                return Err(vec![Finding::new(
                    AGENTS_FILE,
                    None,
                    format!("cannot write {AGENTS_FILE}: {error}"),
                )]);
            }
        }
        Err(error) => {
            return Err(vec![Finding::new(
                AGENTS_FILE,
                None,
                format!("cannot stage {AGENTS_FILE} update: {error}"),
            )]);
        }
    }
    if let Err(error) = std::fs::rename(&tmp_path, target) {
        let _ = std::fs::remove_file(&tmp_path);
        return Err(vec![Finding::new(
            AGENTS_FILE,
            None,
            format!("cannot commit {AGENTS_FILE} update: {error}"),
        )]);
    }
    Ok(())
}

/// Removes files this `init` invocation exclusively created, once a later
/// write fails. Never called with a pre-existing `AGENTS.md`: the upsert
/// path leaves that file untouched on failure instead.
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

/// Result of a successful [`init`]: paths created fresh, and paths that
/// already existed and were modified in place (only ever `AGENTS.md`).
#[derive(Debug, Clone, Default)]
pub struct InitOutcome {
    pub created: Vec<String>,
    pub updated: Vec<String>,
}

/// Initializes a Specful repository: configuration, the artifact directory
/// skeleton, `docs/SPECFUL.md`, and the `AGENTS.md` pointer block. Refuses
/// to touch an already-initialized root.
///
/// Preconditions run before any write, in order: project key,
/// `.specful/config.yaml` absence (an early advisory check purely for error
/// ordering; `create_new` below is the actual enforcement and the
/// concurrency lock), `docs/SPECFUL.md` absence, and `AGENTS.md`
/// well-formedness. A write failure after `.specful/config.yaml` is created
/// rolls back every file this invocation
/// exclusively created; it never touches a pre-existing `AGENTS.md`. If
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
    if root.join(SPECFUL_MD_FILE).exists() {
        return Err(vec![Finding::new(
            SPECFUL_MD_FILE,
            None,
            "file already exists",
        )]);
    }
    let agents_path = root.join(AGENTS_FILE);
    let agents_plan = plan_agents_md(&agents_path)?;

    let config = Config {
        project_key: project_key.to_owned(),
        specful_version: env!("CARGO_PKG_VERSION").to_owned(),
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

    if let Err(findings) = write_specful_md(root) {
        rollback(root, &[CONFIG_FILE]);
        return Err(findings);
    }
    created.push(SPECFUL_MD_FILE.to_owned());

    let mut updated = Vec::new();
    match agents_plan {
        AgentsPlan::Create => match create_agents_md(root) {
            Ok(()) => created.push(AGENTS_FILE.to_owned()),
            Err(findings) => {
                rollback(root, &[CONFIG_FILE, SPECFUL_MD_FILE]);
                return Err(findings);
            }
        },
        AgentsPlan::Upsert(content) => match atomic_replace(&agents_path, &content) {
            Ok(()) => updated.push(AGENTS_FILE.to_owned()),
            Err(findings) => {
                rollback(root, &[CONFIG_FILE, SPECFUL_MD_FILE]);
                return Err(findings);
            }
        },
    }

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
        let mut rollback_targets = vec![CONFIG_FILE, SPECFUL_MD_FILE];
        if created.iter().any(|f| f == AGENTS_FILE) {
            rollback_targets.push(AGENTS_FILE);
        }
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

    Ok(InitOutcome { created, updated })
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
