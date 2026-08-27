// SPDX-License-Identifier: Apache-2.0

use std::process::Command;

fn scratch() -> tempfile::TempDir {
    tempfile::tempdir().expect("create scratch")
}

#[test]
fn validate_discovers_the_root_from_a_nested_working_directory() {
    let root = scratch();
    specful::authoring::init(root.path(), "EXAMPLE").expect("init");
    let nested = root.path().join("docs/specs/backend/sync");
    std::fs::create_dir_all(&nested).expect("create nested dir");

    let output = Command::new(env!("CARGO_BIN_EXE_specful"))
        .args(["validate"])
        .current_dir(&nested)
        .output()
        .expect("run specful validate");

    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn validate_reports_a_clear_error_with_no_ancestor_configuration() {
    // Its own TempDir, not a subdirectory of CARGO_TARGET_TMPDIR: see
    // tests/cli_new_root_discovery.rs for why that location is unsafe for
    // this exact test (this repository's own tree could grow a root
    // .specful/config.yaml and discover_root would find it).
    let root = scratch();

    let output = Command::new(env!("CARGO_BIN_EXE_specful"))
        .args(["validate"])
        .current_dir(root.path())
        .output()
        .expect("run specful validate");

    assert!(
        !output.status.success(),
        "validate must fail with no ancestor configuration"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(".specful/config.yaml"),
        "expected a clear .specful/config.yaml discovery error, got: {stdout}"
    );
}

#[test]
fn show_discovers_the_root_from_a_nested_working_directory() {
    let root = scratch();
    specful::authoring::init(root.path(), "EXAMPLE").expect("init");
    specful::authoring::new_artifact(root.path(), specful::authoring::NewKind::Adr, None, "Seed")
        .expect("new adr");
    let index_findings = specful::index::run_index(root.path(), false);
    assert!(index_findings.is_empty(), "index generation should succeed");
    let nested = root.path().join("docs/specs/backend/sync");
    std::fs::create_dir_all(&nested).expect("create nested dir");

    let output = Command::new(env!("CARGO_BIN_EXE_specful"))
        .args(["show", "EXAMPLE-ADR-0001"])
        .current_dir(&nested)
        .output()
        .expect("run specful show");

    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("id: EXAMPLE-ADR-0001"),
        "expected the discovered root's catalog to be shown, got: {stdout}"
    );
}

#[test]
fn trace_discovers_the_root_from_a_nested_working_directory() {
    // trace's root resolution goes through the same resolve_root helper as
    // show, so this only needs to prove the query pipeline ran against the
    // discovered root (an ADR is a real, meaningful rejection, not a
    // ".specful/config.yaml not found" discovery failure) rather than duplicating
    // full coverage of every trace shape.
    let root = scratch();
    specful::authoring::init(root.path(), "EXAMPLE").expect("init");
    specful::authoring::new_artifact(root.path(), specful::authoring::NewKind::Adr, None, "Seed")
        .expect("new adr");
    let index_findings = specful::index::run_index(root.path(), false);
    assert!(index_findings.is_empty(), "index generation should succeed");
    let nested = root.path().join("docs/specs/backend/sync");
    std::fs::create_dir_all(&nested).expect("create nested dir");

    let output = Command::new(env!("CARGO_BIN_EXE_specful"))
        .args(["trace", "EXAMPLE-ADR-0001"])
        .current_dir(&nested)
        .output()
        .expect("run specful trace");

    assert!(
        !output.status.success(),
        "trace of an ADR must fail, but root discovery must have succeeded"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("trace is not defined for ADRs"),
        "expected the discovered root's catalog to be queried, got: {stdout}"
    );
}

#[test]
fn index_discovers_the_root_from_a_nested_working_directory() {
    let root = scratch();
    specful::authoring::init(root.path(), "EXAMPLE").expect("init");
    // run_index only writes generated views once at least one artifact
    // exists, so plant one before exercising discovery.
    specful::authoring::new_artifact(root.path(), specful::authoring::NewKind::Adr, None, "Seed")
        .expect("new adr");
    let nested = root.path().join("docs/specs/backend/sync");
    std::fs::create_dir_all(&nested).expect("create nested dir");

    let output = Command::new(env!("CARGO_BIN_EXE_specful"))
        .args(["index"])
        .current_dir(&nested)
        .output()
        .expect("run specful index");

    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        root.path().join(".specful/generated/catalog.json").exists(),
        "the generated catalog must land under the discovered ancestor root"
    );
}
