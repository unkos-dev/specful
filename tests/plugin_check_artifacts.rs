// SPDX-License-Identifier: Apache-2.0

//! Behavioural cases for `plugin/scripts/check-artifacts`, fed recorded Claude Code PostToolUse
//! hook JSON on stdin. A stub `specful` binary
//! (`tests/fixtures/plugin/fake-specful`) stands in for the real CLI so exit codes and output are
//! under the test's control.

use serde_json::json;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn script_path() -> PathBuf {
    repo_root().join("plugin/scripts/check-artifacts")
}

/// The directory containing `python3` on this machine's `PATH`, resolved once so the tests can
/// build a `PATH` that finds `python3` without also finding a real `specful` binary.
fn python3_dir() -> PathBuf {
    let path = std::env::var("PATH").unwrap_or_default();
    for dir in std::env::split_paths(&path) {
        if dir.join("python3").is_file() {
            return dir;
        }
    }
    panic!("python3 not found on PATH");
}

fn init_specful_repo(root: &Path) {
    std::fs::create_dir_all(root.join(".specful")).expect("create .specful");
    std::fs::write(
        root.join(".specful/config.yaml"),
        "config-version: 1\n\
         project-key: OK\n\
         specful-version: \"0.1.0\"\n\
         next-adr-sequence: 1\n\
         next-requirement-sequence: 1\n\
         next-design-sequence: 1\n",
    )
    .expect("write config");
}

/// Runs the hook script with a `PostToolUse` payload for editing `file_path` (relative to
/// `cwd`). When `fake_specful` is set, `PATH` is arranged so the stub at
/// `tests/fixtures/plugin/fake-specful` answers as `specful`, controlled by the given
/// `(exit_code, output)`.
fn run_hook(cwd: &Path, file_path: &Path, fake_specful: Option<(&str, &str)>) -> Output {
    let payload = json!({
        "hook_event_name": "PostToolUse",
        "tool_name": "Edit",
        "cwd": cwd.to_string_lossy(),
        "tool_input": {
            "file_path": file_path.to_string_lossy(),
        },
    });

    let mut command = Command::new("python3");
    command
        .arg(script_path())
        .env_clear()
        .current_dir(cwd)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    match fake_specful {
        Some((exit_code, stub_output)) => {
            command
                .env(
                    "PATH",
                    format!(
                        "{}:{}",
                        repo_root().join("tests/fixtures/plugin").display(),
                        python3_dir().display()
                    ),
                )
                .env("FAKE_SPECFUL_EXIT", exit_code)
                .env("FAKE_SPECFUL_OUTPUT", stub_output);
        }
        None => {
            command.env("PATH", python3_dir());
        }
    }

    let mut child = command.spawn().expect("spawn check-artifacts");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(payload.to_string().as_bytes())
        .expect("write hook payload");
    child.wait_with_output().expect("wait for check-artifacts")
}

#[test]
fn stays_silent_for_a_clean_specful_repository() {
    let scratch = tempfile::tempdir().expect("scratch dir");
    let root = scratch.path();
    init_specful_repo(root);
    std::fs::create_dir_all(root.join("docs/specs")).expect("create docs/specs");
    let edited = root.join("docs/specs/example.md");
    std::fs::write(&edited, "content").expect("write artifact");

    let output = run_hook(root, &edited, Some(("0", "")));

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty(), "stderr: {:?}", output.stderr);
}

#[test]
fn reports_findings_for_an_artifact_path_edit_with_drift() {
    let scratch = tempfile::tempdir().expect("scratch dir");
    let root = scratch.path();
    init_specful_repo(root);
    std::fs::create_dir_all(root.join("docs/adr")).expect("create docs/adr");
    let edited = root.join("docs/adr/0001-example.md");
    std::fs::write(&edited, "content").expect("write artifact");

    let output = run_hook(
        root,
        &edited,
        Some(("1", "docs/adr/0001-example.md: missing required section")),
    );

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing required section"),
        "stderr: {stderr}"
    );
}

#[test]
fn stays_silent_for_a_non_artifact_path() {
    let scratch = tempfile::tempdir().expect("scratch dir");
    let root = scratch.path();
    init_specful_repo(root);
    std::fs::create_dir_all(root.join("src")).expect("create src");
    let edited = root.join("src/main.rs");
    std::fs::write(&edited, "fn main() {}").expect("write source");

    let output = run_hook(root, &edited, None);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
}

#[test]
fn stays_silent_for_a_non_specful_repository() {
    let scratch = tempfile::tempdir().expect("scratch dir");
    let root = scratch.path();
    std::fs::create_dir_all(root.join("docs/specs")).expect("create docs/specs");
    let edited = root.join("docs/specs/example.md");
    std::fs::write(&edited, "content").expect("write artifact");

    let output = run_hook(root, &edited, None);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
}

#[test]
fn warns_once_when_the_specful_binary_is_absent() {
    let scratch = tempfile::tempdir().expect("scratch dir");
    let root = scratch.path();
    init_specful_repo(root);
    std::fs::create_dir_all(root.join("docs/specs")).expect("create docs/specs");
    let edited = root.join("docs/specs/example.md");
    std::fs::write(&edited, "content").expect("write artifact");

    let output = run_hook(root, &edited, None);
    // Exit 1, not 2: the warning goes to the user, never into the agent loop.
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr.lines().count(), 1, "stderr: {stderr}");
    assert!(stderr.contains("specful"), "stderr: {stderr}");
}

#[test]
fn never_crashes_on_malformed_stdin() {
    let scratch = tempfile::tempdir().expect("scratch dir");
    let root = scratch.path();

    let mut child = Command::new("python3")
        .arg(script_path())
        .env_clear()
        .env("PATH", python3_dir())
        .current_dir(root)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn check-artifacts");
    child
        .stdin
        .take()
        .expect("stdin piped")
        .write_all(b"not json at all")
        .expect("write malformed payload");
    let output = child.wait_with_output().expect("wait for check-artifacts");

    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
}
