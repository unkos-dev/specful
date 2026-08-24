// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use specful::authoring::{NewKind, init, new_artifact};

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    if dir.exists() {
        std::fs::remove_dir_all(&dir).expect("clear scratch");
    }
    std::fs::create_dir_all(&dir).expect("create scratch");
    dir
}

const LOCK_FILE: &str = ".specful.yaml.lock";

#[test]
fn artifact_collision_is_reported_not_overwritten() {
    let root = scratch("artifact-collision");
    init(&root, "EXAMPLE").expect("init");
    std::fs::create_dir_all(root.join("docs/adr")).expect("create adr dir");
    std::fs::write(root.join("docs/adr/0001-first-decision.md"), "sentinel").expect("plant file");

    let findings =
        new_artifact(&root, NewKind::Adr, None, "First decision").expect_err("must collide");
    assert!(
        findings
            .iter()
            .any(|f| f.message.contains("already exists")),
        "expected an already-exists finding, got {findings:?}"
    );

    let content =
        std::fs::read_to_string(root.join("docs/adr/0001-first-decision.md")).expect("read");
    assert_eq!(
        content, "sentinel",
        "a colliding allocation must never overwrite the existing artifact"
    );
}

#[test]
fn stale_lock_file_is_reported_and_left_for_manual_removal() {
    let root = scratch("stale-lock");
    init(&root, "EXAMPLE").expect("init");
    std::fs::write(root.join(LOCK_FILE), "").expect("plant stale lock");

    let findings =
        new_artifact(&root, NewKind::Adr, None, "Locked decision").expect_err("must be locked");
    assert!(
        findings
            .iter()
            .any(|f| f.message.to_lowercase().contains("lock")),
        "expected a lock-related finding, got {findings:?}"
    );
    assert!(
        root.join(LOCK_FILE).exists(),
        "a stale lock must be left for manual removal, not silently deleted"
    );
}

#[test]
fn lock_file_is_removed_after_successful_allocation() {
    let root = scratch("lock-cleanup-success");
    init(&root, "EXAMPLE").expect("init");
    new_artifact(&root, NewKind::Adr, None, "Cleaned up").expect("create");
    assert!(
        !root.join(LOCK_FILE).exists(),
        "the lock must not survive a successful allocation"
    );
}

#[test]
fn lock_file_is_removed_after_a_failed_allocation() {
    let root = scratch("lock-cleanup-failure");
    init(&root, "EXAMPLE").expect("init");
    std::fs::create_dir_all(root.join("docs/adr")).expect("create adr dir");
    std::fs::write(root.join("docs/adr/0001-first-decision.md"), "sentinel").expect("plant file");

    let _ = new_artifact(&root, NewKind::Adr, None, "First decision");
    assert!(
        !root.join(LOCK_FILE).exists(),
        "the lock must be released even when the artifact write fails"
    );
}

#[test]
fn lock_file_is_removed_via_drop_when_configuration_cannot_be_loaded() {
    let root = scratch("lock-cleanup-config-error");
    init(&root, "EXAMPLE").expect("init");
    // Corrupt the configuration after init so the lock is acquired (the
    // whole read-modify-write is now inside the critical section) but the
    // subsequent load fails before any rename ever consumes the lock file.
    // This is the genuine Drop path, not the rename-already-removed-it path.
    std::fs::write(root.join(".specful.yaml"), "not: [valid").expect("corrupt config");

    let findings = new_artifact(&root, NewKind::Adr, None, "Broken config")
        .expect_err("a broken configuration must fail allocation");
    assert!(!findings.is_empty());
    assert!(
        !root.join(LOCK_FILE).exists(),
        "the lock must be released via Drop when configuration loading fails"
    );
}

#[cfg(unix)]
#[test]
fn symlinked_scope_directory_is_rejected() {
    let root = scratch("symlink-scope");
    init(&root, "EXAMPLE").expect("init");

    let outside = Path::new(env!("CARGO_TARGET_TMPDIR")).join("symlink-scope-outside");
    if outside.exists() {
        std::fs::remove_dir_all(&outside).expect("clear outside dir");
    }
    std::fs::create_dir_all(&outside).expect("create outside dir");

    std::os::unix::fs::symlink(&outside, root.join("docs/specs/backend"))
        .expect("plant scope symlink");

    let findings = new_artifact(&root, NewKind::Msrs, Some("backend"), "Escaping module")
        .expect_err("a symlinked scope directory must be rejected");
    assert!(
        findings.iter().any(|f| f.message == "symlink not allowed"),
        "expected a symlink finding, got {findings:?}"
    );
    assert!(
        std::fs::read_dir(&outside)
            .expect("read outside dir")
            .next()
            .is_none(),
        "nothing must be written outside the repository root"
    );
}
