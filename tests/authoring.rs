// SPDX-License-Identifier: Apache-2.0

use specful::authoring::{NewKind, SPECFUL_MARKER_END, SPECFUL_MARKER_START, init, new_artifact};
use specful::index::CATALOG_PATH;

fn scratch() -> tempfile::TempDir {
    tempfile::tempdir().expect("create scratch")
}

const LOCK_FILE: &str = ".specful/config.yaml.lock";
const AGENTS_FILE: &str = "AGENTS.md";
const SPECFUL_MD_FILE: &str = "docs/SPECFUL.md";

#[test]
fn fresh_init_writes_specful_md_and_agents_md() {
    let root = scratch();
    let outcome = init(root.path(), "EXAMPLE").expect("init");

    assert!(outcome.created.iter().any(|p| p == SPECFUL_MD_FILE));
    assert!(outcome.created.iter().any(|p| p == AGENTS_FILE));
    assert!(outcome.updated.is_empty());

    let specful_md =
        std::fs::read_to_string(root.path().join(SPECFUL_MD_FILE)).expect("read docs/SPECFUL.md");
    assert!(specful_md.starts_with("# This repository uses Specful"));
    assert!(specful_md.contains(concat!(
        "https://github.com/unkos-dev/specful/tree/v",
        env!("CARGO_PKG_VERSION")
    )));
    assert!(!specful_md.contains("{version}"));

    let agents_md = std::fs::read_to_string(root.path().join(AGENTS_FILE)).expect("read AGENTS.md");
    assert!(agents_md.starts_with(SPECFUL_MARKER_START));
    assert!(agents_md.contains(SPECFUL_MARKER_END));
}

#[test]
fn fresh_init_writes_generated_views_for_a_zero_artifact_repository() {
    let root = scratch();
    let outcome = init(root.path(), "EXAMPLE").expect("init");

    assert!(outcome.created.iter().any(|p| p == CATALOG_PATH));
    assert!(outcome.created.iter().any(|p| p == "docs/specs/index.md"));
    assert!(root.path().join(CATALOG_PATH).is_file());
    assert!(root.path().join("docs/specs/index.md").is_file());
}

#[test]
fn failed_init_preserves_author_owned_specs_index() {
    let root = scratch();
    let index_path = root.path().join("docs/specs/index.md");
    std::fs::create_dir_all(index_path.parent().expect("parent")).expect("create docs/specs");
    let author_owned = "# My hand-written index\n";
    std::fs::write(&index_path, author_owned).expect("plant author-owned index");

    init(root.path(), "EXAMPLE").expect_err("init must refuse an author-owned index");

    let after = std::fs::read_to_string(&index_path).expect("author-owned index must survive");
    assert_eq!(after, author_owned);
}

#[test]
fn existing_agents_md_without_markers_gains_appended_block() {
    let root = scratch();
    std::fs::write(
        root.path().join(AGENTS_FILE),
        "# Existing guidance\n\nDo the thing.\n",
    )
    .expect("plant AGENTS.md");

    let outcome = init(root.path(), "EXAMPLE").expect("init");
    assert!(outcome.updated.iter().any(|p| p == AGENTS_FILE));
    assert!(!outcome.created.iter().any(|p| p == AGENTS_FILE));

    let agents_md = std::fs::read_to_string(root.path().join(AGENTS_FILE)).expect("read AGENTS.md");
    assert!(agents_md.starts_with("# Existing guidance\n\nDo the thing.\n"));
    assert!(agents_md.contains(SPECFUL_MARKER_START));
    assert!(agents_md.contains(SPECFUL_MARKER_END));
    assert_eq!(agents_md.matches(SPECFUL_MARKER_START).count(), 1);
    assert_eq!(agents_md.matches(SPECFUL_MARKER_END).count(), 1);
}

#[test]
fn agents_md_with_well_formed_markers_gets_only_between_content_replaced() {
    let root = scratch();
    std::fs::write(
        root.path().join(AGENTS_FILE),
        format!(
            "# Existing guidance\n\n{SPECFUL_MARKER_START}\nstale instructions\n{SPECFUL_MARKER_END}\n\nMore guidance.\n"
        ),
    )
    .expect("plant AGENTS.md");

    let outcome = init(root.path(), "EXAMPLE").expect("init");
    assert!(outcome.updated.iter().any(|p| p == AGENTS_FILE));

    let agents_md = std::fs::read_to_string(root.path().join(AGENTS_FILE)).expect("read AGENTS.md");
    assert!(agents_md.starts_with("# Existing guidance\n\n"));
    assert!(agents_md.ends_with("\n\nMore guidance.\n"));
    assert!(!agents_md.contains("stale instructions"));
    assert_eq!(agents_md.matches(SPECFUL_MARKER_START).count(), 1);
    assert_eq!(agents_md.matches(SPECFUL_MARKER_END).count(), 1);
}

fn assert_malformed_markers_block_init(agents_content: &str) {
    let root = scratch();
    std::fs::write(root.path().join(AGENTS_FILE), agents_content).expect("plant AGENTS.md");

    let findings = init(root.path(), "EXAMPLE").expect_err("malformed markers must be a finding");
    assert!(
        findings.iter().any(|f| f.path == AGENTS_FILE),
        "expected a finding against AGENTS.md, got {findings:?}"
    );
    assert!(
        !root.path().join(".specful/config.yaml").exists(),
        "no write may happen when a precondition fails"
    );
    assert_eq!(
        std::fs::read_to_string(root.path().join(AGENTS_FILE)).expect("read AGENTS.md"),
        agents_content,
        "a rejected AGENTS.md must be left byte-for-byte untouched"
    );

    // A corrected file then allows a clean rerun.
    std::fs::write(root.path().join(AGENTS_FILE), "# Fixed\n").expect("fix AGENTS.md");
    init(root.path(), "EXAMPLE").expect("init succeeds once the file is corrected");
}

#[test]
fn single_start_marker_alone_is_malformed() {
    assert_malformed_markers_block_init(&format!("{SPECFUL_MARKER_START}\nbody\n"));
}

#[test]
fn single_end_marker_alone_is_malformed() {
    assert_malformed_markers_block_init(&format!("body\n{SPECFUL_MARKER_END}\n"));
}

#[test]
fn reversed_markers_are_malformed() {
    assert_malformed_markers_block_init(&format!(
        "{SPECFUL_MARKER_END}\nbody\n{SPECFUL_MARKER_START}\n"
    ));
}

#[test]
fn duplicate_start_marker_is_malformed() {
    assert_malformed_markers_block_init(&format!(
        "{SPECFUL_MARKER_START}\n{SPECFUL_MARKER_START}\nbody\n{SPECFUL_MARKER_END}\n"
    ));
}

#[test]
fn two_complete_marker_pairs_are_malformed() {
    assert_malformed_markers_block_init(&format!(
        "{SPECFUL_MARKER_START}\nfirst\n{SPECFUL_MARKER_END}\n\n\
         {SPECFUL_MARKER_START}\nsecond\n{SPECFUL_MARKER_END}\n"
    ));
}

#[cfg(unix)]
#[test]
fn symlinked_agents_md_is_rejected_and_target_untouched() {
    let root = scratch();
    let outside = scratch();
    let target = outside.path().join("real-agents.md");
    std::fs::write(&target, "original content\n").expect("plant link target");
    std::os::unix::fs::symlink(&target, root.path().join(AGENTS_FILE)).expect("plant symlink");

    let findings =
        init(root.path(), "EXAMPLE").expect_err("a symlinked AGENTS.md must be rejected");
    assert!(
        findings.iter().any(|f| f.message == "symlink not allowed"),
        "expected a symlink finding, got {findings:?}"
    );
    assert!(
        !root.path().join(".specful/config.yaml").exists(),
        "no write may happen when a precondition fails"
    );
    assert_eq!(
        std::fs::read_to_string(&target).expect("read link target"),
        "original content\n",
        "the symlink target must never be modified"
    );
}

#[test]
fn preexisting_docs_specful_md_is_rejected() {
    let root = scratch();
    std::fs::create_dir_all(root.path().join("docs")).expect("create docs dir");
    std::fs::write(root.path().join(SPECFUL_MD_FILE), "local edits\n")
        .expect("plant docs/SPECFUL.md");

    let findings =
        init(root.path(), "EXAMPLE").expect_err("a pre-existing docs/SPECFUL.md must be rejected");
    assert!(
        findings.iter().any(|f| f.path == SPECFUL_MD_FILE),
        "expected a finding naming docs/SPECFUL.md, got {findings:?}"
    );
    assert!(
        !root.path().join(".specful/config.yaml").exists(),
        "no write may happen when a precondition fails"
    );
}

#[test]
fn installed_content_names_no_harness() {
    let root = scratch();
    init(root.path(), "EXAMPLE").expect("init");

    let specful_md = std::fs::read_to_string(root.path().join(SPECFUL_MD_FILE)).expect("read");
    let agents_md = std::fs::read_to_string(root.path().join(AGENTS_FILE)).expect("read");

    for harness in ["claude", "cursor", "copilot", "gemini"] {
        assert!(
            !specful_md.to_lowercase().contains(harness),
            "docs/SPECFUL.md must not name harness {harness}"
        );
        assert!(
            !agents_md.to_lowercase().contains(harness),
            "AGENTS.md must not name harness {harness}"
        );
    }
}

#[test]
fn artifact_collision_is_reported_not_overwritten() {
    let root = scratch();
    init(root.path(), "EXAMPLE").expect("init");
    std::fs::create_dir_all(root.path().join("docs/adr")).expect("create adr dir");
    std::fs::write(
        root.path().join("docs/adr/0001-first-decision.md"),
        "sentinel",
    )
    .expect("plant file");

    let findings =
        new_artifact(root.path(), NewKind::Adr, None, "First decision").expect_err("must collide");
    assert!(
        findings
            .iter()
            .any(|f| f.message.contains("already exists")),
        "expected an already-exists finding, got {findings:?}"
    );

    let content =
        std::fs::read_to_string(root.path().join("docs/adr/0001-first-decision.md")).expect("read");
    assert_eq!(
        content, "sentinel",
        "a colliding allocation must never overwrite the existing artifact"
    );
}

#[test]
fn stale_lock_file_is_reported_and_left_for_manual_removal() {
    let root = scratch();
    init(root.path(), "EXAMPLE").expect("init");
    std::fs::write(root.path().join(LOCK_FILE), "").expect("plant stale lock");

    let findings = new_artifact(root.path(), NewKind::Adr, None, "Locked decision")
        .expect_err("must be locked");
    assert!(
        findings
            .iter()
            .any(|f| f.message.to_lowercase().contains("lock")),
        "expected a lock-related finding, got {findings:?}"
    );
    assert!(
        root.path().join(LOCK_FILE).exists(),
        "a stale lock must be left for manual removal, not silently deleted"
    );
}

#[test]
fn lock_file_is_removed_after_successful_allocation() {
    let root = scratch();
    init(root.path(), "EXAMPLE").expect("init");
    new_artifact(root.path(), NewKind::Adr, None, "Cleaned up").expect("create");
    assert!(
        !root.path().join(LOCK_FILE).exists(),
        "the lock must not survive a successful allocation"
    );
}

#[test]
fn lock_file_is_removed_after_a_failed_allocation() {
    let root = scratch();
    init(root.path(), "EXAMPLE").expect("init");
    std::fs::create_dir_all(root.path().join("docs/adr")).expect("create adr dir");
    std::fs::write(
        root.path().join("docs/adr/0001-first-decision.md"),
        "sentinel",
    )
    .expect("plant file");

    let _ = new_artifact(root.path(), NewKind::Adr, None, "First decision");
    assert!(
        !root.path().join(LOCK_FILE).exists(),
        "the lock must be released even when the artifact write fails"
    );
}

#[test]
fn lock_file_is_removed_via_drop_when_configuration_cannot_be_loaded() {
    let root = scratch();
    init(root.path(), "EXAMPLE").expect("init");
    // Corrupt the configuration after init so the lock is acquired (the
    // whole read-modify-write is now inside the critical section) but the
    // subsequent load fails before any rename ever consumes the lock file.
    // This is the genuine Drop path, not the rename-already-removed-it path.
    std::fs::write(root.path().join(".specful/config.yaml"), "not: [valid")
        .expect("corrupt config");

    let findings = new_artifact(root.path(), NewKind::Adr, None, "Broken config")
        .expect_err("a broken configuration must fail allocation");
    assert!(!findings.is_empty());
    assert!(
        !root.path().join(LOCK_FILE).exists(),
        "the lock must be released via Drop when configuration loading fails"
    );
}

#[cfg(unix)]
#[test]
fn symlinked_scope_directory_is_rejected() {
    let root = scratch();
    init(root.path(), "EXAMPLE").expect("init");

    // A second TempDir rather than a fixed name under CARGO_TARGET_TMPDIR:
    // atomic, randomized creation, and cleanup on drop even if this test
    // panics before reaching the end.
    let outside = scratch();

    std::os::unix::fs::symlink(outside.path(), root.path().join("docs/specs/backend"))
        .expect("plant scope symlink");

    let findings = new_artifact(
        root.path(),
        NewKind::Requirement,
        Some("backend"),
        "Escaping module",
    )
    .expect_err("a symlinked scope directory must be rejected");
    assert!(
        findings.iter().any(|f| f.message == "symlink not allowed"),
        "expected a symlink finding, got {findings:?}"
    );
    assert!(
        std::fs::read_dir(outside.path())
            .expect("read outside dir")
            .next()
            .is_none(),
        "nothing must be written outside the repository root"
    );
}

#[test]
fn a_plain_file_blocking_the_scope_path_is_reported_as_not_a_directory() {
    let root = scratch();
    init(root.path(), "EXAMPLE").expect("init");

    std::fs::write(root.path().join("docs/specs/backend"), "not a directory")
        .expect("plant blocking file");

    let findings = new_artifact(
        root.path(),
        NewKind::Requirement,
        Some("backend"),
        "Blocked module",
    )
    .expect_err("a plain file blocking the scope path must be rejected");
    assert!(
        findings.iter().any(|f| f.message == "not a directory"),
        "expected a not-a-directory finding, got {findings:?}"
    );
    assert!(
        !findings.iter().any(|f| f.message == "symlink not allowed"),
        "a plain file is not a symlink and must not be reported as one, got {findings:?}"
    );
}

#[cfg(unix)]
#[test]
fn init_rejects_a_symlinked_docs_directory() {
    let root = scratch();
    let outside = scratch();
    std::os::unix::fs::symlink(outside.path(), root.path().join("docs"))
        .expect("plant docs symlink");

    let findings = init(root.path(), "EXAMPLE").expect_err("a symlinked docs dir must be rejected");
    assert!(
        findings.iter().any(|f| f.message == "symlink not allowed"),
        "expected a symlink finding, got {findings:?}"
    );
    assert!(
        std::fs::read_dir(outside.path())
            .expect("read outside dir")
            .next()
            .is_none(),
        "nothing must be written outside the repository root"
    );
}

#[cfg(unix)]
#[test]
fn init_rejects_a_dangling_config_symlink() {
    let root = scratch();
    let outside = scratch();
    std::fs::create_dir_all(root.path().join(".specful")).expect("create .specful dir");
    let target = outside.path().join("nonexistent/config.yaml");
    std::os::unix::fs::symlink(&target, root.path().join(".specful/config.yaml"))
        .expect("plant dangling config symlink");

    let findings = init(root.path(), "EXAMPLE")
        .expect_err("a dangling config symlink must be rejected, not written through");
    assert!(
        findings
            .iter()
            .any(|f| f.message == "repository is already initialized"),
        "expected the already-initialized finding (the symlink entry itself already exists at \
         .specful/config.yaml), got {findings:?}"
    );
    assert!(
        !target.parent().expect("target has a parent").exists(),
        "nothing must be created through the dangling symlink's target directory"
    );
}

#[cfg(unix)]
#[test]
fn init_leaves_created_directories_after_a_config_write_failure() {
    // A dangling symlink at .specful/config.yaml makes the exclusive config
    // create fail (the symlink entry itself already exists) after the
    // directories were already created. init does not roll those
    // directories back: they are empty and harmless, and a rerun
    // completes the job.
    let root = scratch();
    let outside = scratch();
    std::fs::create_dir_all(root.path().join(".specful")).expect("create .specful dir");
    std::os::unix::fs::symlink(
        outside.path().join("nonexistent/config.yaml"),
        root.path().join(".specful/config.yaml"),
    )
    .expect("plant dangling config symlink");

    let findings = init(root.path(), "EXAMPLE").expect_err("config write must fail");
    assert!(!findings.is_empty());
    assert!(
        root.path().join("docs/adr").is_dir(),
        "the ADR directory created before the failure must remain"
    );
    assert!(
        root.path().join("docs/specs").is_dir(),
        "the specs directory created before the failure must remain"
    );
}

/// Applies the same line-prefix substitution `new_artifact` uses, so a
/// scaffold's body can be checked against its template independent of the
/// private implementation: only the allocated id, the title (frontmatter and
/// H1), and (ADR only) `recorded-on` may differ from the template.
fn expected_scaffold(
    template_path: &str,
    id: &str,
    title: &str,
    recorded_on: Option<&str>,
) -> String {
    let template = std::fs::read_to_string(template_path).expect("read template");
    let mut replacements: Vec<(&str, String)> = vec![
        ("id: \"", format!("{id}\"")),
        ("title: \"", format!("{title}\"")),
    ];
    if let Some(date) = recorded_on {
        replacements.push(("recorded-on: \"", format!("{date}\"")));
    }
    replacements.push(("# ", title.to_owned()));

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

fn template_path(name: &str) -> String {
    format!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/{}"), name)
}

/// Today's date, matching the format `new_artifact` stamps into an ADR's
/// `recorded-on`: kept independent of `authoring`'s private `today` so this
/// test exercises the CLI's actual output, not its own helper.
fn today() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock is after 1970")
        .as_secs();
    let days = (seconds / 86_400) as i64;
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    let year = if month <= 2 { year + 1 } else { year };
    format!("{year:04}-{month:02}-{day:02}")
}

#[test]
fn new_adr_scaffold_equals_its_template_after_exact_substitutions() {
    let root = scratch();
    init(root.path(), "EXAMPLE").expect("init");
    let path =
        new_artifact(root.path(), NewKind::Adr, None, "Adopt event replay").expect("new adr");
    let content = std::fs::read_to_string(root.path().join(&path)).expect("read scaffold");
    let expected = expected_scaffold(
        &template_path("adr.md"),
        "EXAMPLE-ADR-0001",
        "Adopt event replay",
        Some(&today()),
    );
    assert_eq!(content, expected);
}

#[test]
fn new_requirement_scaffold_equals_its_template_after_exact_substitutions() {
    let root = scratch();
    init(root.path(), "EXAMPLE").expect("init");
    let path = new_artifact(
        root.path(),
        NewKind::Requirement,
        Some("backend/sync"),
        "Sync requirements",
    )
    .expect("new requirement");
    let content = std::fs::read_to_string(root.path().join(&path)).expect("read scaffold");
    let expected = expected_scaffold(
        &template_path("requirement.md"),
        "EXAMPLE-REQ-0001",
        "Sync requirements",
        None,
    );
    assert_eq!(content, expected);
}

#[test]
fn new_design_scaffold_equals_its_template_after_exact_substitutions() {
    let root = scratch();
    init(root.path(), "EXAMPLE").expect("init");
    let path = new_artifact(
        root.path(),
        NewKind::Design,
        Some("backend/sync"),
        "Sync design",
    )
    .expect("new design");
    let content = std::fs::read_to_string(root.path().join(&path)).expect("read scaffold");
    let expected = expected_scaffold(
        &template_path("design.md"),
        "EXAMPLE-DESIGN-0001",
        "Sync design",
        None,
    );
    assert_eq!(content, expected);
}

/// A freshly scaffolded artifact has real content but unresolved template
/// guidance; it must fail validation. The repository's own completed
/// artifacts, by contrast, must pass.
#[test]
fn fresh_scaffolds_fail_validation_and_the_repository_itself_passes() {
    let root = scratch();
    init(root.path(), "EXAMPLE").expect("init");
    new_artifact(root.path(), NewKind::Adr, None, "Adopt event replay").expect("new adr");
    new_artifact(
        root.path(),
        NewKind::Requirement,
        Some("backend/sync"),
        "Sync requirements",
    )
    .expect("new requirement");
    new_artifact(
        root.path(),
        NewKind::Design,
        Some("backend/sync"),
        "Sync design",
    )
    .expect("new design");

    let findings = specful::repo::validate_repository(root.path());
    assert!(
        !findings.is_empty(),
        "a freshly scaffolded repository must fail validation on unresolved template residue"
    );

    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_findings = specful::repo::validate_repository(repo_root);
    assert!(
        repo_findings.is_empty(),
        "this repository's own completed artifacts must pass validation, got:\n{}",
        repo_findings
            .iter()
            .map(|f| f.render())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
