// SPDX-License-Identifier: Apache-2.0

use std::process::Command;

fn scratch() -> tempfile::TempDir {
    tempfile::tempdir().expect("create scratch")
}

/// Overwrites a freshly scaffolded ADR with schema-conformant, residue-free
/// content: these tests exercise root discovery, not scaffold completeness,
/// and a raw scaffold's unresolved optional frontmatter placeholders are
/// expected to fail collection.
fn complete_adr(root: &std::path::Path, path: &str) {
    std::fs::write(
        root.join(path),
        "---\n\
         type: ADR\n\
         profile-version: 1\n\
         id: EXAMPLE-ADR-0001\n\
         title: Seed\n\
         status: accepted\n\
         recorded-on: 2026-08-30\n\
         decision-makers:\n\
         \x20 - John\n\
         ---\n\
         \n\
         # Seed\n\
         \n\
         ## Context and problem statement\n\
         \n\
         Root discovery needs a conformant ADR to index.\n\
         \n\
         ## Decision drivers\n\
         \n\
         - A discovery test needs one collectible artifact.\n\
         \n\
         ## Considered options\n\
         \n\
         - Seed a conformant ADR.\n\
         \n\
         ## Decision outcome\n\
         \n\
         Chosen option: seed a conformant ADR, because it lets discovery run without also asserting scaffold shape.\n\
         \n\
         ### Consequences\n\
         \n\
         Positive: discovery tests stay independent of scaffold content.\n\
         \n\
         ### Confirmation\n\
         \n\
         This file collects and indexes without findings.\n",
    )
    .expect("write conformant seed ADR");
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
    let adr_path = specful::authoring::new_artifact(
        root.path(),
        specful::authoring::NewKind::Adr,
        None,
        "Seed",
    )
    .expect("new adr");
    complete_adr(root.path(), &adr_path);
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
    let adr_path = specful::authoring::new_artifact(
        root.path(),
        specful::authoring::NewKind::Adr,
        None,
        "Seed",
    )
    .expect("new adr");
    complete_adr(root.path(), &adr_path);
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
    let adr_path = specful::authoring::new_artifact(
        root.path(),
        specful::authoring::NewKind::Adr,
        None,
        "Seed",
    )
    .expect("new adr");
    complete_adr(root.path(), &adr_path);
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
