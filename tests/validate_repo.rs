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
