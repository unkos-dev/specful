// SPDX-License-Identifier: Apache-2.0

use std::path::Path;

use specful::repo::validate_repository;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn flags_every_planted_defect_in_the_invalid_repository() {
    let findings = validate_repository(&fixture("invalid-repo"));
    let rendered: Vec<String> = findings.iter().map(|f| f.render()).collect();
    let combined = rendered.join("\n");

    let expected = [
        "\"superseded-by\" is a required property",
        "filename sequence 0001 does not match identifier BAD-MSRS-0002",
        "governed-by target BAD-ADR-0009 does not exist",
        "satisfies target BAD-REQ-0404 does not exist",
        "next-msrs-sequence 1 lags allocated identifier sequence 2",
        "next-requirement-sequence 1 lags allocated identifier sequence 1",
    ];
    for needle in expected {
        assert!(
            combined.contains(needle),
            "missing expected finding {needle:?} in:\n{combined}"
        );
    }
}

#[test]
fn accepts_the_valid_repository() {
    let findings = validate_repository(&fixture("valid-repo"));
    let rendered: Vec<String> = findings.iter().map(|f| f.render()).collect();
    assert!(
        findings.is_empty(),
        "expected no findings, got:\n{}",
        rendered.join("\n")
    );
}

/// Copies a fixture tree into a fresh temporary directory and returns it.
fn copy_fixture(name: &str, label: &str) -> std::path::PathBuf {
    fn copy_tree(from: &Path, to: &Path) {
        std::fs::create_dir_all(to).expect("create directory");
        for entry in std::fs::read_dir(from).expect("read fixture directory") {
            let entry = entry.expect("read fixture entry");
            let target = to.join(entry.file_name());
            if entry.file_type().expect("entry type").is_dir() {
                copy_tree(&entry.path(), &target);
            } else {
                std::fs::copy(entry.path(), &target).expect("copy fixture file");
            }
        }
    }

    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock is after the epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("specful-{label}-{unique}"));
    let _ = std::fs::remove_dir_all(&root);
    copy_tree(&fixture(name), &root);
    root
}

#[cfg(unix)]
#[test]
fn reports_symlinks_without_following_them() {
    let root = copy_fixture("valid-repo", "symlink");
    // Points back at an ancestor: following it would never terminate.
    std::os::unix::fs::symlink("../../..", root.join("docs/specs/system/loop"))
        .expect("create symlink");

    let findings = validate_repository(&root);
    let rendered: Vec<String> = findings.iter().map(|f| f.render()).collect();
    std::fs::remove_dir_all(&root).expect("remove temporary repository");

    assert!(
        rendered
            .iter()
            .any(|f| f == "docs/specs/system/loop: symlink not allowed"),
        "expected a symlink finding, got:\n{}",
        rendered.join("\n")
    );
}

#[cfg(unix)]
#[test]
fn reports_unreadable_directories() {
    use std::os::unix::fs::PermissionsExt;

    /// Restores readable permissions even when the assertions panic.
    struct Restore(std::path::PathBuf);
    impl Drop for Restore {
        fn drop(&mut self) {
            let _ = std::fs::set_permissions(&self.0, std::fs::Permissions::from_mode(0o755));
            let _ =
                std::fs::remove_dir_all(self.0.parent().and_then(Path::parent).unwrap_or(&self.0));
        }
    }

    let root = copy_fixture("valid-repo", "unreadable");
    let locked = root.join("docs/adr");
    std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o000))
        .expect("remove directory permissions");
    let _restore = Restore(locked);

    let findings = validate_repository(&root);
    let rendered: Vec<String> = findings.iter().map(|f| f.render()).collect();
    assert!(
        rendered
            .iter()
            .any(|f| f.starts_with("docs/adr: cannot read directory")),
        "expected an unreadable directory finding, got:\n{}",
        rendered.join("\n")
    );
}
