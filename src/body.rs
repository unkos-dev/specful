//! Markdown body structure checks for the artifact profiles.
//!
//! Checks are deliberately line-based rather than a full Markdown parse:
//! every rule in this profile is anchored to whole lines (ATX headings,
//! fence delimiters), so a line scan is sufficient and keeps the checker
//! free of a Markdown-parsing dependency.

use std::collections::HashSet;

use crate::diagnostics::Finding;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactKind {
    Adr,
    Msrs,
    Msdd,
}

/// One body line plus its file-relative position and fence membership.
struct Line<'a> {
    file_line: usize,
    text: &'a str,
    /// Whether this line sits inside a fenced code block, including the
    /// delimiter line itself: an info string on an opening fence (for
    /// example ` ```{r} `) can otherwise match a placeholder pattern.
    in_fence: bool,
}

/// Returns the fence character (backtick or tilde) a line opens or closes
/// with, or `None` when the line is not a fence delimiter.
fn fence_char(line: &str) -> Option<char> {
    let trimmed = line.trim_start();
    if trimmed.starts_with("```") {
        Some('`')
    } else if trimmed.starts_with("~~~") {
        Some('~')
    } else {
        None
    }
}

fn scan_lines(body: &str, body_first_line: usize) -> Vec<Line<'_>> {
    let mut lines = Vec::new();
    let mut open_fence: Option<char> = None;
    for (i, raw) in body.lines().enumerate() {
        let file_line = body_first_line + i;
        if let Some(marker) = fence_char(raw) {
            // A fence only closes with the same character it opened with,
            // so a mismatched marker inside an open fence is ordinary
            // fenced content rather than a delimiter.
            match open_fence {
                Some(current) if current == marker => open_fence = None,
                Some(_) => {}
                None => open_fence = Some(marker),
            }
            lines.push(Line {
                file_line,
                text: raw,
                in_fence: true,
            });
            continue;
        }
        lines.push(Line {
            file_line,
            text: raw,
            in_fence: open_fence.is_some(),
        });
    }
    lines
}

/// Returns the ATX heading level and trimmed text, or `None` when the line
/// is not a valid heading (wrong marker count, or no space after `#`s).
fn heading_level_and_text(line: &str) -> Option<(usize, &str)> {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i] == b'#' {
        i += 1;
    }
    if i == 0 || i > 6 {
        return None;
    }
    if i >= bytes.len() || bytes[i] != b' ' {
        return None;
    }
    Some((i, line[i + 1..].trim()))
}

/// The level-one heading's text, trimmed of trailing whitespace only, per
/// the title-parity rule. Callers must already know `line` is a level-one
/// heading (its first two bytes are `"# "`).
fn h1_text_trailing_trim(line: &str) -> &str {
    line[2..].trim_end()
}

/// Whether `text` contains `word` as a standalone token: neither the
/// character before nor after the match is ASCII alphanumeric.
fn contains_word_token(text: &str, word: &str) -> bool {
    let bytes = text.as_bytes();
    let mut start = 0;
    while start < text.len() {
        let Some(rel) = text[start..].find(word) else {
            return false;
        };
        let abs = start + rel;
        let before_ok = abs == 0 || !(bytes[abs - 1] as char).is_ascii_alphanumeric();
        let after = abs + word.len();
        let after_ok = after >= bytes.len() || !(bytes[after] as char).is_ascii_alphanumeric();
        if before_ok && after_ok {
            return true;
        }
        start = abs + 1;
    }
    false
}

const BCP14_TERMS: [&str; 7] = [
    "MUST",
    "REQUIRED",
    "SHALL",
    "SHOULD",
    "RECOMMENDED",
    "MAY",
    "OPTIONAL",
];

fn contains_bcp14_term(text: &str) -> bool {
    BCP14_TERMS
        .iter()
        .any(|term| contains_word_token(text, term))
}

/// Whether `text` contains one non-empty `{...}` pair on a single line, the
/// bracket placeholder convention used throughout `templates/adr.md`.
fn has_brace_placeholder(text: &str) -> bool {
    if let Some(open) = text.find('{')
        && let Some(close_rel) = text[open + 1..].find('}')
    {
        return close_rel > 0;
    }
    false
}

/// Whether `text` still carries template placeholder or instructional
/// residue. Markers are taken verbatim from the templates: bracket
/// placeholders (`{...}`), the bare `NNNN` sequence number token, and
/// instructional HTML comments. The SPDX license comment in
/// `templates/adr.md` is excluded: it is a machine-readable license notice
/// documented in `README.md`, not authoring guidance.
fn has_placeholder_residue(text: &str) -> bool {
    if text.contains("<!--") && !text.contains("SPDX-License-Identifier") {
        return true;
    }
    if text.contains("NNNN") {
        return true;
    }
    has_brace_placeholder(text)
}

fn placeholder_findings(lines: &[Line<'_>], path: &str) -> Vec<Finding> {
    lines
        .iter()
        .filter(|l| !l.in_fence && has_placeholder_residue(l.text))
        .map(|l| {
            Finding::new(
                path,
                Some(l.file_line),
                "line still contains template placeholder residue",
            )
        })
        .collect()
}

/// Recursively scans a frontmatter value's string scalars for the same
/// `{...}` bracket placeholder convention `has_brace_placeholder` checks in
/// the body, so an unfilled frontmatter field (for example the ADR
/// `decision-makers` scaffold entry) is not silently accepted just because
/// placeholder scanning only ever looked at the Markdown body. A YAML flow
/// mapping such as the MSRS `requirements: ID: {}` scaffold entry parses to
/// a JSON object, never a string, so it is never mistaken for a
/// placeholder; only actual string scalars are checked.
fn frontmatter_placeholder_findings(frontmatter: &serde_json::Value, path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    collect_frontmatter_placeholders(frontmatter, "", path, &mut findings);
    findings
}

fn collect_frontmatter_placeholders(
    value: &serde_json::Value,
    pointer: &str,
    path: &str,
    findings: &mut Vec<Finding>,
) {
    match value {
        serde_json::Value::String(text) => {
            if has_brace_placeholder(text) {
                let location = pointer.strip_prefix('/').unwrap_or(pointer);
                findings.push(Finding::new(
                    path,
                    None,
                    format!("{location} still contains template placeholder residue"),
                ));
            }
        }
        serde_json::Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                collect_frontmatter_placeholders(
                    item,
                    &format!("{pointer}/{index}"),
                    path,
                    findings,
                );
            }
        }
        serde_json::Value::Object(map) => {
            for (key, item) in map {
                collect_frontmatter_placeholders(item, &format!("{pointer}/{key}"), path, findings);
            }
        }
        _ => {}
    }
}

/// Splits a requirement heading's text at the first `": "` into an id and a
/// title. A heading with no separator is treated as an id-only heading, so
/// it still participates in id-set comparison rather than being dropped.
fn split_id_title(text: &str) -> &str {
    match text.split_once(": ") {
        Some((id, _title)) => id,
        None => text,
    }
}

fn check_common(
    lines: &[Line<'_>],
    frontmatter: &serde_json::Value,
    path: &str,
    body_first_line: usize,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    let h1s: Vec<&Line<'_>> = lines
        .iter()
        .filter(|l| !l.in_fence && heading_level_and_text(l.text).is_some_and(|(lvl, _)| lvl == 1))
        .collect();

    match h1s.len() {
        0 => findings.push(Finding::new(
            path,
            Some(body_first_line),
            "missing a level-one heading",
        )),
        1 => {
            let h1 = h1s[0];
            let text = h1_text_trailing_trim(h1.text);
            if let Some(title) = frontmatter.get("title").and_then(|v| v.as_str())
                && text != title
            {
                findings.push(Finding::new(
                    path,
                    Some(h1.file_line),
                    "level-one heading does not match frontmatter title",
                ));
            }
        }
        _ => {
            for h in &h1s {
                findings.push(Finding::new(
                    path,
                    Some(h.file_line),
                    "multiple level-one headings found",
                ));
            }
        }
    }

    findings.extend(placeholder_findings(lines, path));
    findings.extend(frontmatter_placeholder_findings(frontmatter, path));
    findings
}

fn check_msrs(lines: &[Line<'_>], frontmatter: &serde_json::Value, path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    let req_sections: Vec<(usize, &Line<'_>)> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| !l.in_fence && heading_level_and_text(l.text) == Some((2, "Requirements")))
        .collect();

    if req_sections.is_empty() {
        findings.push(Finding::new(
            path,
            None,
            "missing a level-two \"Requirements\" section",
        ));
        return findings;
    }
    if req_sections.len() > 1 {
        for (_, l) in &req_sections {
            findings.push(Finding::new(
                path,
                Some(l.file_line),
                "multiple level-two \"Requirements\" sections found",
            ));
        }
        return findings;
    }

    let (start_idx, req_heading) = req_sections[0];
    let mut end_idx = lines.len();
    for (idx, l) in lines.iter().enumerate().skip(start_idx + 1) {
        if l.in_fence {
            continue;
        }
        if let Some((lvl, _)) = heading_level_and_text(l.text)
            && lvl <= 2
        {
            end_idx = idx;
            break;
        }
    }
    let section_start = start_idx + 1;

    struct Block {
        id: String,
        heading_line: usize,
        start: usize,
        end: usize,
    }

    let mut h3_positions: Vec<(usize, &str, usize)> = Vec::new();
    for (idx, l) in lines.iter().enumerate().take(end_idx).skip(section_start) {
        if l.in_fence {
            continue;
        }
        if let Some((lvl, text)) = heading_level_and_text(l.text)
            && lvl == 3
        {
            h3_positions.push((idx, text, l.file_line));
        }
    }

    let mut blocks = Vec::new();
    for (i, (idx, text, file_line)) in h3_positions.iter().enumerate() {
        let block_end = h3_positions
            .get(i + 1)
            .map_or(end_idx, |(next_idx, _, _)| *next_idx);
        blocks.push(Block {
            id: split_id_title(text).to_string(),
            heading_line: *file_line,
            start: idx + 1,
            end: block_end,
        });
    }

    let mut seen = HashSet::new();
    for b in &blocks {
        if !seen.insert(b.id.clone()) {
            findings.push(Finding::new(
                path,
                Some(b.heading_line),
                format!("duplicate requirement heading \"{}\"", b.id),
            ));
        }
    }

    if let Some(reqs) = frontmatter.get("requirements").and_then(|v| v.as_object()) {
        let heading_ids: HashSet<&str> = blocks.iter().map(|b| b.id.as_str()).collect();
        for b in &blocks {
            if !reqs.contains_key(&b.id) {
                findings.push(Finding::new(
                    path,
                    Some(b.heading_line),
                    format!(
                        "requirement heading \"{}\" has no matching frontmatter entry",
                        b.id
                    ),
                ));
            }
        }
        for key in reqs.keys() {
            if !heading_ids.contains(key.as_str()) {
                findings.push(Finding::new(
                    path,
                    Some(req_heading.file_line),
                    format!("frontmatter requirement \"{key}\" has no matching heading"),
                ));
            }
        }
    }

    for b in &blocks {
        let block_lines = &lines[b.start..b.end];
        // The obligation lives in the normative paragraph, before the first
        // subsection. Guidance under "#### Verification" or "#### Rationale"
        // discusses the requirement rather than stating it.
        let normative_end = block_lines
            .iter()
            .position(|l| {
                !l.in_fence && heading_level_and_text(l.text).is_some_and(|(lvl, _)| lvl >= 4)
            })
            .unwrap_or(block_lines.len());
        let normative = &block_lines[..normative_end];

        let has_bcp14 = normative
            .iter()
            .any(|l| !l.in_fence && contains_bcp14_term(l.text));
        if !has_bcp14 {
            findings.push(Finding::new(
                path,
                Some(b.heading_line),
                "requirement block has no normative BCP 14 term before its first subsection",
            ));
        }

        let has_should = normative
            .iter()
            .any(|l| !l.in_fence && contains_word_token(l.text, "SHOULD"));
        if has_should {
            let has_rationale = block_lines.iter().any(|l| {
                !l.in_fence
                    && heading_level_and_text(l.text)
                        .is_some_and(|(lvl, text)| lvl == 4 && text == "Rationale")
            });
            if !has_rationale {
                findings.push(Finding::new(
                    path,
                    Some(b.heading_line),
                    "requirement block uses SHOULD/SHOULD NOT but has no \"#### Rationale\" section",
                ));
            }
        }
    }

    findings
}

fn check_msdd(lines: &[Line<'_>], path: &str, body_first_line: usize) -> Vec<Finding> {
    let h1_line = lines
        .iter()
        .find(|l| !l.in_fence && heading_level_and_text(l.text).is_some_and(|(lvl, _)| lvl == 1))
        .map(|l| l.file_line);

    let has_content = lines.iter().any(|l| {
        if l.text.trim().is_empty() {
            return false;
        }
        Some(l.file_line) != h1_line
    });

    if has_content {
        Vec::new()
    } else {
        vec![Finding::new(
            path,
            Some(h1_line.unwrap_or(body_first_line)),
            "body has no content besides the heading",
        )]
    }
}

/// Finds the first line index after `after_idx` (exclusive) whose heading
/// level is `<= level`, marking the end of that section's span. Returns
/// `lines.len()` when no such heading follows.
fn section_end(lines: &[Line<'_>], after_idx: usize, level: usize) -> usize {
    for (idx, l) in lines.iter().enumerate().skip(after_idx + 1) {
        if l.in_fence {
            continue;
        }
        if let Some((lvl, _)) = heading_level_and_text(l.text)
            && lvl <= level
        {
            return idx;
        }
    }
    lines.len()
}

/// A section has content when it contains any non-blank line that is not
/// itself a heading. Nested subsection prose counts toward the parent.
fn section_nonempty(lines: &[Line<'_>], start_idx: usize, end_idx: usize) -> bool {
    lines[start_idx + 1..end_idx].iter().any(|l| {
        if l.text.trim().is_empty() {
            return false;
        }
        if !l.in_fence && heading_level_and_text(l.text).is_some() {
            return false;
        }
        true
    })
}

const ADR_REQUIRED_H2: [&str; 4] = [
    "Context and Problem Statement",
    "Decision Drivers",
    "Considered Options",
    "Decision Outcome",
];

const ADR_OPTIONAL_H2: [&str; 2] = ["Pros and Cons of the Options", "More Information"];

const ADR_REQUIRED_H3: [&str; 2] = ["Consequences", "Confirmation"];

fn check_adr(lines: &[Line<'_>], path: &str) -> Vec<Finding> {
    let mut findings = Vec::new();

    let h2s: Vec<(usize, &str, usize)> = lines
        .iter()
        .enumerate()
        .filter_map(|(idx, l)| {
            if l.in_fence {
                return None;
            }
            heading_level_and_text(l.text)
                .filter(|(lvl, _)| *lvl == 2)
                .map(|(_, text)| (idx, text, l.file_line))
        })
        .collect();

    for name in ADR_REQUIRED_H2 {
        let matches: Vec<_> = h2s.iter().filter(|(_, t, _)| *t == name).collect();
        if matches.is_empty() {
            findings.push(Finding::new(
                path,
                None,
                format!("missing required section \"## {name}\""),
            ));
        } else if matches.len() > 1 {
            for (_, _, fl) in &matches {
                findings.push(Finding::new(
                    path,
                    Some(*fl),
                    format!("duplicate \"## {name}\" section"),
                ));
            }
        }
    }

    let mut encountered: Vec<(usize, usize)> = Vec::new();
    for (_, text, fl) in &h2s {
        if let Some(req_idx) = ADR_REQUIRED_H2.iter().position(|n| n == text)
            && !encountered.iter().any(|(ri, _)| *ri == req_idx)
        {
            encountered.push((req_idx, *fl));
        }
    }
    let mut max_seen = 0usize;
    let mut started = false;
    for (req_idx, fl) in &encountered {
        if started && *req_idx < max_seen {
            findings.push(Finding::new(
                path,
                Some(*fl),
                format!(
                    "\"## {}\" section is out of order",
                    ADR_REQUIRED_H2[*req_idx]
                ),
            ));
        }
        max_seen = max_seen.max(*req_idx);
        started = true;
    }

    for (idx, text, fl) in &h2s {
        if ADR_REQUIRED_H2.contains(text) || ADR_OPTIONAL_H2.contains(text) {
            let end = section_end(lines, *idx, 2);
            if !section_nonempty(lines, *idx, end) {
                findings.push(Finding::new(
                    path,
                    Some(*fl),
                    format!("\"## {text}\" section has no content"),
                ));
            }
        }
    }

    // Consequences and Confirmation are level-three subsections of
    // Decision Outcome. Scope the search to that section's span so a
    // stray top-level heading of the same name is not mistaken for it.
    let decision_outcome = h2s.iter().find(|(_, t, _)| *t == "Decision Outcome");
    let (scope_start, scope_end) = match decision_outcome {
        Some((idx, _, _)) => (*idx, section_end(lines, *idx, 2)),
        None => (0, lines.len()),
    };

    let h3s: Vec<(usize, &str, usize)> = lines[scope_start..scope_end]
        .iter()
        .enumerate()
        .filter_map(|(rel_idx, l)| {
            if l.in_fence {
                return None;
            }
            heading_level_and_text(l.text)
                .filter(|(lvl, _)| *lvl == 3)
                .map(|(_, text)| (scope_start + rel_idx, text, l.file_line))
        })
        .collect();

    for name in ADR_REQUIRED_H3 {
        let matches: Vec<_> = h3s.iter().filter(|(_, t, _)| *t == name).collect();
        if matches.is_empty() {
            findings.push(Finding::new(
                path,
                decision_outcome.map(|(idx, _, _)| lines[*idx].file_line),
                format!("missing required section \"### {name}\""),
            ));
        } else if matches.len() > 1 {
            for (_, _, fl) in &matches {
                findings.push(Finding::new(
                    path,
                    Some(*fl),
                    format!("duplicate \"### {name}\" section"),
                ));
            }
        }
    }

    let mut encountered3: Vec<(usize, usize)> = Vec::new();
    for (_, text, fl) in &h3s {
        if let Some(ri) = ADR_REQUIRED_H3.iter().position(|n| n == text)
            && !encountered3.iter().any(|(r, _)| *r == ri)
        {
            encountered3.push((ri, *fl));
        }
    }
    let mut max_seen3 = 0usize;
    let mut started3 = false;
    for (ri, fl) in &encountered3 {
        if started3 && *ri < max_seen3 {
            findings.push(Finding::new(
                path,
                Some(*fl),
                format!("\"### {}\" section is out of order", ADR_REQUIRED_H3[*ri]),
            ));
        }
        max_seen3 = max_seen3.max(*ri);
        started3 = true;
    }

    for (idx, text, fl) in &h3s {
        let end = section_end(lines, *idx, 3);
        if !section_nonempty(lines, *idx, end) {
            findings.push(Finding::new(
                path,
                Some(*fl),
                format!("\"### {text}\" section has no content"),
            ));
        }
    }

    findings
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
    let lines = scan_lines(body, body_first_line);

    let mut findings = check_common(&lines, frontmatter, path, body_first_line);

    match kind {
        ArtifactKind::Msrs => findings.extend(check_msrs(&lines, frontmatter, path)),
        ArtifactKind::Msdd => findings.extend(check_msdd(&lines, path, body_first_line)),
        ArtifactKind::Adr => findings.extend(check_adr(&lines, path)),
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn findings_for(
        kind: ArtifactKind,
        frontmatter: serde_json::Value,
        body: &str,
    ) -> Vec<Finding> {
        check_body(kind, &frontmatter, body, "docs/example.md", 1)
    }

    // ---- common: heading count ----

    #[test]
    fn missing_h1_is_a_finding() {
        let fm = json!({});
        let findings = findings_for(ArtifactKind::Msdd, fm, "no heading here\n");
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("missing a level-one heading"))
        );
    }

    #[test]
    fn multiple_h1_is_a_finding_per_heading() {
        let fm = json!({});
        let body = "# First\n\nsome text\n\n# Second\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        let multi: Vec<_> = findings
            .iter()
            .filter(|f| f.message.contains("multiple level-one headings"))
            .collect();
        assert_eq!(multi.len(), 2);
        assert_eq!(multi[0].line, Some(1));
        assert_eq!(multi[1].line, Some(5));
    }

    #[test]
    fn fenced_fake_heading_is_ignored() {
        let fm = json!({"title": "Real title"});
        let body = "# Real title\n\n```\n# not a heading\n```\n\nsome content\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        assert!(findings.is_empty());
    }

    // ---- common: title parity ----

    #[test]
    fn title_mismatch_is_a_finding_on_h1_line() {
        let fm = json!({"title": "Expected Title"});
        let body = "line offset\n# Wrong Title\n\nbody content\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        let mismatch = findings
            .iter()
            .find(|f| f.message.contains("does not match frontmatter title"))
            .expect("mismatch finding");
        assert_eq!(mismatch.line, Some(2));
    }

    #[test]
    fn title_match_trims_trailing_whitespace_only() {
        let fm = json!({"title": "Exact Title"});
        let body = "# Exact Title   \n\nbody content\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("does not match"))
        );
    }

    #[test]
    fn missing_title_field_skips_parity_check() {
        let fm = json!({});
        let body = "# Any Heading\n\nbody content\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("does not match"))
        );
    }

    // ---- common: placeholder residue ----

    #[test]
    fn brace_placeholder_is_flagged() {
        let fm = json!({"title": "T"});
        let body = "# T\n\n{Fill this in}\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("placeholder residue") && f.line == Some(3))
        );
    }

    #[test]
    fn bare_nnnn_token_is_flagged() {
        let fm = json!({"title": "T"});
        let body = "# T\n\nPROJECT-REQ-NNNN needs a real id\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("placeholder residue") && f.line == Some(3))
        );
    }

    #[test]
    fn instructional_comment_is_flagged() {
        let fm = json!({"title": "T"});
        let body = "# T\n\n<!-- Replace this placeholder. -->\n\nbody\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("placeholder residue") && f.line == Some(3))
        );
    }

    #[test]
    fn spdx_comment_is_not_flagged() {
        let fm = json!({"title": "T"});
        let body = "<!-- SPDX-License-Identifier: CC0-1.0 -->\n\n# T\n\nbody content\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("placeholder residue"))
        );
    }

    #[test]
    fn placeholder_inside_fence_is_ignored() {
        let fm = json!({"title": "T"});
        let body = "# T\n\n```\n{still a placeholder pattern}\n```\n\nbody content\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("placeholder residue"))
        );
    }

    #[test]
    fn fence_info_string_is_not_a_placeholder() {
        let fm = json!({"title": "T"});
        let body = "# T\n\n```{r}\ncode\n```\n\nbody content\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("placeholder residue"))
        );
    }

    #[test]
    fn frontmatter_string_scalar_placeholder_is_flagged() {
        let fm = json!({
            "title": "T",
            "decision-makers": ["{Decision maker}"],
        });
        let body = "# T\n\nbody content\n";
        let findings = findings_for(ArtifactKind::Adr, fm, body);
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("decision-makers")
                    && f.message.contains("placeholder residue")
                    && f.line.is_none()),
            "expected a line-less frontmatter placeholder finding naming decision-makers, got {findings:?}"
        );
    }

    #[test]
    fn frontmatter_flow_mapping_braces_are_not_a_placeholder() {
        // `requirements: ID: {}` parses to a JSON object, not a string
        // scalar, so the empty-mapping syntax must never be mistaken for
        // `{...}` placeholder residue.
        let fm = json!({
            "title": "T",
            "requirements": {"OK-REQ-0001": {}},
        });
        let body = "# T\n\n## Requirements\n\n### OK-REQ-0001: Title\n\nThe system MUST do it.\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("placeholder residue")),
            "an empty flow mapping must not be flagged, got {findings:?}"
        );
    }

    #[test]
    fn heading_inside_tilde_fence_is_ignored() {
        let fm = json!({"title": "T"});
        let body = "# T\n\n~~~\n# heading-looking-text\n~~~\n\nbody content\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        assert!(!findings.iter().any(|f| f.message.contains("heading")));
    }

    // ---- common: line offset arithmetic ----

    #[test]
    fn findings_use_body_first_line_offset() {
        let fm = json!({});
        let findings = check_body(ArtifactKind::Msdd, &fm, "no heading\n", "p.md", 42);
        let f = findings
            .iter()
            .find(|f| f.message.contains("missing a level-one heading"))
            .unwrap();
        assert_eq!(f.line, Some(42));

        let fm2 = json!({"title": "Late Title"});
        let body2 = "filler 1\nfiller 2\n# Wrong\n";
        let findings2 = check_body(ArtifactKind::Msdd, &fm2, body2, "p.md", 10);
        let f2 = findings2
            .iter()
            .find(|f| f.message.contains("does not match"))
            .unwrap();
        // body line 3 with body_first_line 10 -> file line 12
        assert_eq!(f2.line, Some(12));
    }

    // ---- MSDD ----

    #[test]
    fn msdd_empty_body_is_a_finding() {
        let fm = json!({"title": "T"});
        let body = "# T\n\n\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("no content besides the heading"))
        );
    }

    #[test]
    fn msdd_nonempty_body_has_no_content_finding() {
        let fm = json!({"title": "T"});
        let body = "# T\n\nSome design prose.\n";
        let findings = findings_for(ArtifactKind::Msdd, fm, body);
        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("no content besides"))
        );
    }

    // ---- MSRS ----

    fn msrs_frontmatter() -> serde_json::Value {
        json!({
            "title": "Module requirements",
            "requirements": {
                "PROJECT-REQ-0001": {},
                "PROJECT-REQ-0002": {}
            }
        })
    }

    #[test]
    fn msrs_missing_requirements_section_is_a_finding() {
        let fm = msrs_frontmatter();
        let body = "# Module requirements\n\nno requirements section here\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        assert!(findings.iter().any(|f| {
            f.message
                .contains("missing a level-two \"Requirements\" section")
        }));
    }

    #[test]
    fn msrs_multiple_requirements_sections_is_a_finding() {
        let fm = msrs_frontmatter();
        let body = "# Module requirements\n\n## Requirements\n\n### PROJECT-REQ-0001: One\n\nThe system MUST do it.\n\n## Requirements\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        let multi: Vec<_> = findings
            .iter()
            .filter(|f| {
                f.message
                    .contains("multiple level-two \"Requirements\" sections")
            })
            .collect();
        assert_eq!(multi.len(), 2);
    }

    #[test]
    fn msrs_heading_without_mapping_entry_is_a_finding() {
        let fm = msrs_frontmatter();
        let body = "# Module requirements\n\n## Requirements\n\n### PROJECT-REQ-0001: One\n\nThe system MUST do it.\n\n### PROJECT-REQ-0002: Two\n\nThe system MUST do it too.\n\n### PROJECT-REQ-9999: Stray\n\nThe system MUST also do this.\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("PROJECT-REQ-9999")
                    && f.message.contains("no matching frontmatter entry"))
        );
    }

    #[test]
    fn msrs_mapping_key_without_heading_is_a_finding() {
        let fm = msrs_frontmatter();
        let body = "# Module requirements\n\n## Requirements\n\n### PROJECT-REQ-0001: One\n\nThe system MUST do it.\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("PROJECT-REQ-0002")
                    && f.message.contains("no matching heading"))
        );
    }

    #[test]
    fn msrs_duplicate_heading_is_a_finding() {
        let mut fm = msrs_frontmatter();
        fm["requirements"] = json!({"PROJECT-REQ-0001": {}});
        let body = "# Module requirements\n\n## Requirements\n\n### PROJECT-REQ-0001: One\n\nThe system MUST do it.\n\n### PROJECT-REQ-0001: One again\n\nThe system MUST also do it.\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("duplicate requirement heading"))
        );
    }

    #[test]
    fn msrs_block_missing_bcp14_term_is_a_finding() {
        let mut fm = msrs_frontmatter();
        fm["requirements"] = json!({"PROJECT-REQ-0001": {}});
        let body = "# Module requirements\n\n## Requirements\n\n### PROJECT-REQ-0001: One\n\nThis block has no normative language at all.\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("no normative BCP 14 term"))
        );
    }

    #[test]
    fn msrs_block_with_bcp14_term_has_no_finding() {
        let mut fm = msrs_frontmatter();
        fm["requirements"] = json!({"PROJECT-REQ-0001": {}});
        let body = "# Module requirements\n\n## Requirements\n\n### PROJECT-REQ-0001: One\n\nThe system MUST validate the input.\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("no normative BCP 14 term"))
        );
    }

    #[test]
    fn msrs_bcp14_term_only_in_a_subsection_is_a_finding() {
        let mut fm = msrs_frontmatter();
        fm["requirements"] = json!({"PROJECT-REQ-0001": {}});
        let body = "# Module requirements\n\n## Requirements\n\n### PROJECT-REQ-0001: One\n\nThe system validates the input.\n\n#### Verification\n\nThe input MUST be rejected in the parser test.\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("no normative BCP 14 term"))
        );
    }

    #[test]
    fn msrs_subsections_without_a_bcp14_term_have_no_finding() {
        let mut fm = msrs_frontmatter();
        fm["requirements"] = json!({"PROJECT-REQ-0001": {}});
        let body = "# Module requirements\n\n## Requirements\n\n### PROJECT-REQ-0001: One\n\nThe system MUST validate the input.\n\n#### Verification\n\nThe parser test covers rejected input.\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    }

    #[test]
    fn msrs_should_only_in_a_subsection_needs_no_rationale() {
        let mut fm = msrs_frontmatter();
        fm["requirements"] = json!({"PROJECT-REQ-0001": {}});
        let body = "# Module requirements\n\n## Requirements\n\n### PROJECT-REQ-0001: One\n\nThe system MUST validate the input.\n\n#### Verification\n\nReviewers SHOULD run the parser test.\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    }

    #[test]
    fn msrs_should_without_rationale_is_a_finding() {
        let mut fm = msrs_frontmatter();
        fm["requirements"] = json!({"PROJECT-REQ-0001": {}});
        let body = "# Module requirements\n\n## Requirements\n\n### PROJECT-REQ-0001: One\n\nThe system SHOULD prefer this approach.\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        assert!(
            findings
                .iter()
                .any(|f| f.message.contains("no \"#### Rationale\" section"))
        );
    }

    #[test]
    fn msrs_should_not_with_rationale_has_no_finding() {
        let mut fm = msrs_frontmatter();
        fm["requirements"] = json!({"PROJECT-REQ-0001": {}});
        let body = "# Module requirements\n\n## Requirements\n\n### PROJECT-REQ-0001: One\n\nThe system SHOULD NOT do this by default.\n\n#### Rationale\n\nBecause of reasons.\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        assert!(!findings.iter().any(|f| f.message.contains("Rationale")));
    }

    #[test]
    fn msrs_fenced_fake_req_heading_is_ignored() {
        let mut fm = msrs_frontmatter();
        fm["requirements"] = json!({"PROJECT-REQ-0001": {}});
        let body = "# Module requirements\n\n## Requirements\n\n### PROJECT-REQ-0001: One\n\nThe system MUST do it.\n\n```\n### PROJECT-REQ-9999: Fake\n```\n";
        let findings = findings_for(ArtifactKind::Msrs, fm, body);
        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("PROJECT-REQ-9999"))
        );
    }

    // ---- ADR ----

    fn adr_frontmatter() -> serde_json::Value {
        json!({"title": "Use PostgreSQL"})
    }

    fn complete_adr_body() -> String {
        [
            "# Use PostgreSQL",
            "",
            "## Context and Problem Statement",
            "",
            "We need a database.",
            "",
            "## Decision Drivers",
            "",
            "- Operational maturity",
            "",
            "## Considered Options",
            "",
            "- PostgreSQL",
            "",
            "## Decision Outcome",
            "",
            "Chosen option: PostgreSQL.",
            "",
            "### Consequences",
            "",
            "- Positive: mature tooling",
            "",
            "### Confirmation",
            "",
            "Reviewed at launch.",
            "",
        ]
        .join("\n")
    }

    #[test]
    fn adr_complete_body_has_no_structural_findings() {
        let fm = adr_frontmatter();
        let findings = findings_for(ArtifactKind::Adr, fm, &complete_adr_body());
        assert!(findings.is_empty(), "{findings:?}");
    }

    #[test]
    fn adr_missing_required_section_is_a_finding() {
        let fm = adr_frontmatter();
        let body =
            complete_adr_body().replace("## Decision Drivers\n\n- Operational maturity\n\n", "");
        let findings = findings_for(ArtifactKind::Adr, fm, &body);
        assert!(findings.iter().any(|f| {
            f.message
                .contains("missing required section \"## Decision Drivers\"")
        }));
    }

    #[test]
    fn adr_missing_nested_section_is_a_finding() {
        let fm = adr_frontmatter();
        let body = complete_adr_body().replace("### Confirmation\n\nReviewed at launch.\n", "");
        let findings = findings_for(ArtifactKind::Adr, fm, &body);
        assert!(findings.iter().any(|f| {
            f.message
                .contains("missing required section \"### Confirmation\"")
        }));
    }

    #[test]
    fn adr_empty_section_is_a_finding() {
        let fm = adr_frontmatter();
        let body = complete_adr_body().replace("We need a database.\n\n", "");
        let findings = findings_for(ArtifactKind::Adr, fm, &body);
        assert!(findings.iter().any(|f| {
            f.message
                .contains("\"## Context and Problem Statement\" section has no content")
        }));
    }

    #[test]
    fn adr_out_of_order_section_is_a_finding() {
        let fm = adr_frontmatter();
        let body = [
            "# Use PostgreSQL",
            "",
            "## Decision Drivers",
            "",
            "- Operational maturity",
            "",
            "## Context and Problem Statement",
            "",
            "We need a database.",
            "",
            "## Considered Options",
            "",
            "- PostgreSQL",
            "",
            "## Decision Outcome",
            "",
            "Chosen option: PostgreSQL.",
            "",
            "### Consequences",
            "",
            "- Positive: mature tooling",
            "",
            "### Confirmation",
            "",
            "Reviewed at launch.",
            "",
        ]
        .join("\n");
        let findings = findings_for(ArtifactKind::Adr, fm, &body);
        assert!(findings.iter().any(|f| {
            f.message
                .contains("\"## Context and Problem Statement\" section is out of order")
        }));
    }

    #[test]
    fn adr_duplicate_section_is_a_finding() {
        let fm = adr_frontmatter();
        let mut body = complete_adr_body();
        body.push_str("## Considered Options\n\n- Another option\n");
        let findings = findings_for(ArtifactKind::Adr, fm, &body);
        assert!(findings.iter().any(|f| {
            f.message
                .contains("duplicate \"## Considered Options\" section")
        }));
    }

    #[test]
    fn adr_optional_section_present_but_empty_is_a_finding() {
        let fm = adr_frontmatter();
        let mut body = complete_adr_body();
        body.push_str("## More Information\n\n");
        let findings = findings_for(ArtifactKind::Adr, fm, &body);
        assert!(findings.iter().any(|f| {
            f.message
                .contains("\"## More Information\" section has no content")
        }));
    }

    #[test]
    fn adr_optional_section_absent_has_no_finding() {
        let fm = adr_frontmatter();
        let findings = findings_for(ArtifactKind::Adr, fm, &complete_adr_body());
        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("More Information"))
        );
    }

    #[test]
    fn adr_optional_section_present_and_filled_has_no_finding() {
        let fm = adr_frontmatter();
        let mut body = complete_adr_body();
        body.push_str("## More Information\n\nSome supporting evidence.\n");
        let findings = findings_for(ArtifactKind::Adr, fm, &body);
        assert!(
            !findings
                .iter()
                .any(|f| f.message.contains("More Information"))
        );
    }
}
