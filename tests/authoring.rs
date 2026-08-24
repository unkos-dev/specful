// SPDX-License-Identifier: Apache-2.0

use specful::authoring::{NewKind, init, new_artifact};

fn scratch() -> tempfile::TempDir {
    tempfile::tempdir().expect("create scratch")
}

const LOCK_FILE: &str = ".specful.yaml.lock";

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
    std::fs::write(root.path().join(".specful.yaml"), "not: [valid").expect("corrupt config");

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
        NewKind::Msrs,
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
        NewKind::Msrs,
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
    let target = outside.path().join("nonexistent/.specful.yaml");
    std::os::unix::fs::symlink(&target, root.path().join(".specful.yaml"))
        .expect("plant dangling config symlink");

    let findings = init(root.path(), "EXAMPLE")
        .expect_err("a dangling config symlink must be rejected, not written through");
    assert!(
        findings
            .iter()
            .any(|f| f.message == "repository is already initialized"),
        "expected the already-initialized finding (the symlink entry itself already exists at \
         .specful.yaml), got {findings:?}"
    );
    assert!(
        !target.parent().expect("target has a parent").exists(),
        "nothing must be created through the dangling symlink's target directory"
    );
}

#[cfg(unix)]
#[test]
fn init_leaves_created_directories_after_a_config_write_failure() {
    // A dangling symlink at .specful.yaml makes the exclusive config
    // create fail (the symlink entry itself already exists) after the
    // directories were already created. init does not roll those
    // directories back: they are empty and harmless, and a rerun
    // completes the job.
    let root = scratch();
    let outside = scratch();
    std::os::unix::fs::symlink(
        outside.path().join("nonexistent/.specful.yaml"),
        root.path().join(".specful.yaml"),
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
