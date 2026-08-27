// SPDX-License-Identifier: Apache-2.0

use std::process::Command;

fn scratch() -> tempfile::TempDir {
    tempfile::tempdir().expect("create scratch")
}

#[test]
fn new_discovers_the_root_from_a_nested_working_directory() {
    let root = scratch();
    specful::authoring::init(root.path(), "EXAMPLE").expect("init");
    let nested = root.path().join("docs/specs/backend/sync");
    std::fs::create_dir_all(&nested).expect("create nested dir");

    let output = Command::new(env!("CARGO_BIN_EXE_specful"))
        .args(["new", "adr", "--title", "Discovered from nested dir"])
        .current_dir(&nested)
        .output()
        .expect("run specful new");

    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        root.path()
            .join("docs/adr/0001-discovered-from-nested-dir.md")
            .exists(),
        "the artifact must land under the discovered ancestor root"
    );
}

#[test]
fn new_reports_a_clear_error_with_no_ancestor_configuration() {
    // Deliberately its own TempDir rather than a subdirectory of
    // CARGO_TARGET_TMPDIR, which lives inside this repository's own tree:
    // if this repository ever gains a root .specful/config.yaml (e.g.
    // dogfooding), discover_root would find it from a scratch dir under
    // the repo and this test would create a real artifact here instead of
    // exercising the no-ancestor error. TempDir gives an atomically and
    // randomly named directory under the system temp location, with RAII
    // cleanup on drop.
    let root = scratch();

    let output = Command::new(env!("CARGO_BIN_EXE_specful"))
        .args(["new", "adr", "--title", "Nothing here"])
        .current_dir(root.path())
        .output()
        .expect("run specful new");

    assert!(
        !output.status.success(),
        "new must fail with no ancestor configuration"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(".specful/config.yaml"),
        "expected a clear .specful/config.yaml discovery error, got: {stdout}"
    );
}
