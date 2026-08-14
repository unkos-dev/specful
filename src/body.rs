//! Markdown body structure checks for the artifact profiles.

use crate::diagnostics::Finding;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactKind {
    Adr,
    Msrs,
    Msdd,
}

/// Checks the Markdown body of one artifact against its profile's
/// structural rules.
///
/// `frontmatter` is the already-loaded metadata value. `body_first_line` is
/// the one-based line in the file where the body begins.
pub fn check_body(
    kind: ArtifactKind,
    frontmatter: &serde_json::Value,
    body: &str,
    path: &str,
    body_first_line: usize,
) -> Vec<Finding> {
    let _ = (kind, frontmatter, body, path, body_first_line);
    todo!("implemented in the body-structure change")
}
