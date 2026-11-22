// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::Path;
use std::process::Command;

const PROFILES: [(&str, &str); 3] = [
    ("adr", "ADR"),
    ("design", "Design"),
    ("requirement", "Requirement"),
];

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn split_site_frontmatter(source: &str) -> (&str, &str) {
    let rest = source
        .strip_prefix("---\n")
        .expect("site profile must start with YAML frontmatter");
    let (frontmatter, body) = rest
        .split_once("\n---\n\n")
        .expect("site profile frontmatter must end with closing delimiter and blank line");
    (frontmatter, body)
}

fn normalise_site_links(body: &str) -> String {
    body.replace("](/specful/profiles/adr/#", "](adr.md#")
        .replace("](/specful/profiles/requirement/)", "](requirement.md)")
        .replace("](/specful/profiles/design/)", "](design.md)")
        .replace("](/specful/", "](https://unkos-dev.github.io/specful/")
}

#[test]
fn profile_command_prints_exact_canonical_profiles_without_repository() {
    let bare = tempfile::tempdir().expect("create bare tempdir");

    for (kind, _) in PROFILES {
        let output = Command::new(env!("CARGO_BIN_EXE_specful"))
            .args(["profile", kind])
            .current_dir(bare.path())
            .output()
            .expect("run specful profile");

        assert!(
            output.status.success(),
            "specful profile {kind} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let expected = fs::read_to_string(repo_root().join(format!("profiles/{kind}.md")))
            .expect("read canonical profile");
        assert_eq!(
            String::from_utf8(output.stdout).expect("valid utf-8 stdout"),
            expected,
            "CLI output for {kind} must match profiles/{kind}.md byte-for-byte"
        );
    }
}

#[test]
fn profile_command_rejects_unknown_kind() {
    let bare = tempfile::tempdir().expect("create bare tempdir");

    let output = Command::new(env!("CARGO_BIN_EXE_specful"))
        .args(["profile", "plan"])
        .current_dir(bare.path())
        .output()
        .expect("run specful profile with invalid kind");

    assert!(
        !output.status.success(),
        "unknown profile kind must fail, got stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

#[test]
fn site_profiles_match_canonical_profiles() {
    for (kind, expected_title) in PROFILES {
        let canonical = fs::read_to_string(repo_root().join(format!("profiles/{kind}.md")))
            .expect("read canonical profile");
        let expected_h1 = format!("# {expected_title}\n\n");
        let canonical_body = canonical
            .strip_prefix(&expected_h1)
            .unwrap_or_else(|| panic!("profiles/{kind}.md must start with {expected_h1:?}"));

        let site = fs::read_to_string(
            repo_root().join(format!("site/src/content/docs/profiles/{kind}.md")),
        )
        .expect("read site profile");
        let (frontmatter, site_body) = split_site_frontmatter(&site);
        let expected_title_line = format!("title: {expected_title}");
        assert!(
            frontmatter.lines().any(|line| line == expected_title_line),
            "site profile for {kind} must declare `{expected_title_line}` in frontmatter"
        );
        assert_eq!(
            normalise_site_links(site_body),
            canonical_body,
            "site/src/content/docs/profiles/{kind}.md body drifted from profiles/{kind}.md"
        );
    }
}
