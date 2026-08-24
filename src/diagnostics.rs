//! Human-readable validation findings.
//!
//! A finding is a plain location plus message. There are no rule codes,
//! severity policies, or stable machine contracts; the `--json` listing is
//! explicitly unstable.

use serde_json::json;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// Repository-relative path using `/` separators.
    pub path: String,
    /// One-based line number when a precise location is known.
    pub line: Option<usize>,
    pub message: String,
}

impl Finding {
    #[must_use]
    pub fn new(path: impl Into<String>, line: Option<usize>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            line,
            message: message.into(),
        }
    }

    #[must_use]
    pub fn render(&self) -> String {
        match self.line {
            Some(line) => format!("{}:{line}: {}", self.path, self.message),
            None => format!("{}: {}", self.path, self.message),
        }
    }

    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "path": self.path,
            "line": self.line,
            "message": self.message,
        })
    }
}

/// Sorts findings source-first: path, then line, then message.
pub fn sort_findings(findings: &mut [Finding]) {
    findings.sort_by(|a, b| {
        a.path
            .cmp(&b.path)
            .then(a.line.unwrap_or(0).cmp(&b.line.unwrap_or(0)))
            .then(a.message.cmp(&b.message))
    });
}
