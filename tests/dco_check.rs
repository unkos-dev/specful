// SPDX-License-Identifier: Apache-2.0

//! Fixtures for scripts/dco-check.sh: valid, missing, mismatched,
//! malformed, and misplaced sign-offs.

use std::path::{Path, PathBuf};
use std::process::Command;

fn git(repo: &Path, args: &[&str]) {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .env("GIT_AUTHOR_NAME", "Test Author")
        .env("GIT_AUTHOR_EMAIL", "author@example.com")
        .env("GIT_COMMITTER_NAME", "Test Author")
        .env("GIT_COMMITTER_EMAIL", "author@example.com")
        .status()
        .expect("git runs");
    assert!(status.success(), "git {args:?} failed");
}

fn commit(repo: &Path, file: &str, message: &str) {
    std::fs::write(repo.join(file), file).expect("write file");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", message]);
}

fn check(repo: &Path) -> bool {
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scripts/dco-check.sh");
    Command::new(script)
        .current_dir(repo)
        .args(["HEAD~1", "HEAD"])
        .status()
        .expect("script runs")
        .success()
}

fn scratch_repo(name: &str) -> PathBuf {
    let repo = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    if repo.exists() {
        std::fs::remove_dir_all(&repo).expect("clear scratch");
    }
    std::fs::create_dir_all(&repo).expect("create scratch");
    git(&repo, &["init", "-q", "-b", "main"]);
    commit(
        &repo,
        "base",
        "chore: base\n\nSigned-off-by: Test Author <author@example.com>",
    );
    repo
}

#[test]
fn accepts_a_matching_trailer_sign_off() {
    let repo = scratch_repo("dco-valid");
    commit(
        &repo,
        "a",
        "feat: signed\n\nSigned-off-by: Test Author <author@example.com>",
    );
    assert!(check(&repo));
}

#[test]
fn rejects_a_missing_sign_off() {
    let repo = scratch_repo("dco-missing");
    commit(&repo, "a", "feat: unsigned");
    assert!(!check(&repo));
}

#[test]
fn rejects_a_sign_off_by_someone_else() {
    let repo = scratch_repo("dco-mismatch");
    commit(
        &repo,
        "a",
        "feat: mismatched\n\nSigned-off-by: Somebody Else <else@example.com>",
    );
    assert!(!check(&repo));
}

#[test]
fn rejects_a_malformed_identity() {
    let repo = scratch_repo("dco-malformed");
    commit(&repo, "a", "feat: malformed\n\nSigned-off-by: <@>");
    assert!(!check(&repo));
}

#[test]
fn rejects_a_sign_off_outside_the_trailer_block() {
    let repo = scratch_repo("dco-misplaced");
    commit(
        &repo,
        "a",
        "feat: misplaced\n\nSigned-off-by: Test Author <author@example.com>\n\nA closing prose paragraph, so the sign-off above sits mid-message\nrather than in the trailer block git parses.",
    );
    assert!(!check(&repo));
}
