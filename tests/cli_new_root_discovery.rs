// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use std::process::Command;

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    if dir.exists() {
        std::fs::remove_dir_all(&dir).expect("clear scratch");
    }
    std::fs::create_dir_all(&dir).expect("create scratch");
    dir
}

#[test]
fn new_discovers_the_root_from_a_nested_working_directory() {
    let root = scratch("nested-root-discovery");
    specful::authoring::init(&root, "EXAMPLE").expect("init");
    let nested = root.join("docs/specs/backend/sync");
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
        root.join("docs/adr/0001-discovered-from-nested-dir.md")
            .exists(),
        "the artifact must land under the discovered ancestor root"
    );
}

#[test]
fn new_reports_a_clear_error_with_no_ancestor_configuration() {
    // Deliberately outside CARGO_TARGET_TMPDIR, which lives inside this
    // repository's own tree: if this repository ever gains a root
    // .specful.yaml (e.g. dogfooding), discover_root would find it from a
    // scratch dir under the repo and this test would create a real
    // artifact here instead of exercising the no-ancestor error.
    let root = std::env::temp_dir().join(format!(
        "specful-no-ancestor-configuration-{}",
        std::process::id()
    ));
    if root.exists() {
        std::fs::remove_dir_all(&root).expect("clear scratch");
    }
    std::fs::create_dir_all(&root).expect("create scratch");

    let output = Command::new(env!("CARGO_BIN_EXE_specful"))
        .args(["new", "adr", "--title", "Nothing here"])
        .current_dir(&root)
        .output()
        .expect("run specful new");

    std::fs::remove_dir_all(&root).expect("clean up scratch");

    assert!(
        !output.status.success(),
        "new must fail with no ancestor configuration"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(".specful.yaml"),
        "expected a clear .specful.yaml discovery error, got: {stdout}"
    );
}
