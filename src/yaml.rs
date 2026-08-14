//! Restricted YAML loading for frontmatter and repository configuration.
//!
//! Specful accepts a deliberately small YAML subset. Anchors, aliases, tags,
//! merge keys, complex keys, duplicate keys, and empty unquoted scalars are
//! loading errors. Plain scalars resolve under the restricted schema: exact
//! lowercase `null`, `true`, and `false` and RFC 8259 numbers take their JSON
//! values; every other non-empty plain scalar is a string. Quoted scalars are
//! always strings.

use crate::diagnostics::Finding;

/// Loads restricted YAML into a JSON-compatible value.
///
/// `path` names the containing file for findings. `first_line` is the
/// one-based line in that file where `source` begins, so finding lines match
/// the file rather than the extracted fragment.
pub fn load_restricted_yaml(
    source: &str,
    path: &str,
    first_line: usize,
) -> Result<serde_json::Value, Vec<Finding>> {
    let _ = (source, path, first_line);
    todo!("implemented in the restricted-YAML change")
}
