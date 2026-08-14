//! Frontmatter extraction for Markdown artifacts.

use crate::diagnostics::Finding;

#[derive(Debug, Clone, Copy)]
pub struct Split<'a> {
    pub yaml: &'a str,
    /// One-based file line where the YAML fragment begins.
    pub yaml_first_line: usize,
    pub body: &'a str,
    /// One-based file line where the body begins.
    pub body_first_line: usize,
}

/// Splits a Markdown artifact into its YAML frontmatter and body.
///
/// The file must open with `---` on its first line and close the
/// frontmatter with a matching `---` line.
pub fn split_frontmatter<'a>(source: &'a str, path: &str) -> Result<Split<'a>, Vec<Finding>> {
    let mut lines = source.split_inclusive('\n');
    let Some(first) = lines.next() else {
        return Err(vec![Finding::new(path, Some(1), "file is empty")]);
    };
    if first.trim_end_matches(['\r', '\n']) != "---" {
        return Err(vec![Finding::new(
            path,
            Some(1),
            "frontmatter must open with --- on the first line",
        )]);
    }

    let mut offset = first.len();
    let yaml_start = offset;
    let mut line = 1;
    for segment in lines {
        line += 1;
        if segment.trim_end_matches(['\r', '\n']) == "---" {
            let yaml = &source[yaml_start..offset];
            let body_start = offset + segment.len();
            return Ok(Split {
                yaml,
                yaml_first_line: 2,
                body: &source[body_start..],
                body_first_line: line + 1,
            });
        }
        offset += segment.len();
    }

    Err(vec![Finding::new(
        path,
        Some(line),
        "frontmatter is never closed with ---",
    )])
}

#[cfg(test)]
mod tests {
    use super::split_frontmatter;

    #[test]
    fn splits_frontmatter_and_body() {
        let source = "---\ntitle: A\n---\n# A\nbody\n";
        let split = split_frontmatter(source, "doc.md").expect("should split");
        assert_eq!(split.yaml, "title: A\n");
        assert_eq!(split.yaml_first_line, 2);
        assert_eq!(split.body, "# A\nbody\n");
        assert_eq!(split.body_first_line, 4);
    }

    #[test]
    fn rejects_missing_opening_marker() {
        let findings = split_frontmatter("# A\n", "doc.md").expect_err("should fail");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].line, Some(1));
    }

    #[test]
    fn rejects_unclosed_frontmatter() {
        let findings = split_frontmatter("---\ntitle: A\n", "doc.md").expect_err("should fail");
        assert_eq!(findings.len(), 1);
        assert!(findings[0].message.contains("never closed"));
    }

    #[test]
    fn rejects_empty_file() {
        assert!(split_frontmatter("", "doc.md").is_err());
    }
}
