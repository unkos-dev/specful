//! Restricted YAML loading for frontmatter and repository configuration.
//!
//! Specful accepts a deliberately small YAML subset. Anchors, aliases, tags,
//! merge keys, complex keys, duplicate keys, and empty unquoted scalars are
//! loading errors. Plain scalars resolve under the restricted schema: exact
//! lowercase `null`, `true`, and `false` and RFC 8259 numbers take their JSON
//! values; every other non-empty plain scalar is a string. Quoted scalars are
//! always strings.

use crate::diagnostics::Finding;
use saphyr_parser::{Event, Marker, Parser, ScalarStyle, ScanError, Span, Tag};
use std::borrow::Cow;

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
    let mut findings = Vec::new();
    let mut parser = Parser::new_from_str(source);

    // The parser always opens with StreamStart; there is nothing to check
    // and no line to attribute if that invariant were ever violated.
    match parser.next() {
        Some(Ok((Event::StreamStart, _))) => {}
        Some(Err(err)) => {
            findings.push(scan_error_finding(path, first_line, &err));
            return Err(findings);
        }
        _ => {
            findings.push(Finding::new(path, None, "unable to start parsing yaml"));
            return Err(findings);
        }
    }

    match parser.next() {
        Some(Ok((Event::DocumentStart(_), _))) => {}
        Some(Ok((Event::StreamEnd, _))) => {
            findings.push(Finding::new(
                path,
                None,
                "expected exactly one yaml document, found none",
            ));
            return Err(findings);
        }
        Some(Err(err)) => {
            findings.push(scan_error_finding(path, first_line, &err));
            return Err(findings);
        }
        _ => {
            findings.push(Finding::new(
                path,
                None,
                "expected exactly one yaml document, found none",
            ));
            return Err(findings);
        }
    }

    let value = match next_node(&mut parser, &mut findings, path, first_line) {
        Ok(value) => value,
        Err(()) => return Err(findings),
    };

    match parser.next() {
        Some(Ok((Event::DocumentEnd, _))) => {}
        Some(Err(err)) => {
            findings.push(scan_error_finding(path, first_line, &err));
            return Err(findings);
        }
        _ => {
            findings.push(Finding::new(path, None, "malformed end of yaml document"));
            return Err(findings);
        }
    }

    // Anything besides an immediate StreamEnd means a second document
    // started; report it and stop, per the "collect what you can" allowance.
    loop {
        match parser.next() {
            Some(Ok((Event::StreamEnd, _))) => break,
            Some(Ok((Event::DocumentStart(_), span))) => {
                findings.push(Finding::new(
                    path,
                    Some(file_line(span.start, first_line)),
                    "expected exactly one yaml document, found multiple",
                ));
                break;
            }
            Some(Ok(_)) => continue,
            Some(Err(err)) => {
                findings.push(scan_error_finding(path, first_line, &err));
                break;
            }
            None => break,
        }
    }

    if findings.is_empty() {
        Ok(value)
    } else {
        Err(findings)
    }
}

/// Converts a fragment-relative marker line into a file line.
fn file_line(marker: Marker, first_line: usize) -> usize {
    marker.line() + first_line - 1
}

fn scan_error_finding(path: &str, first_line: usize, err: &ScanError) -> Finding {
    Finding::new(
        path,
        Some(file_line(*err.marker(), first_line)),
        err.info().to_string(),
    )
}

/// Pulls the next event from the parser and builds the value it introduces.
///
/// Returns `Err(())` only when the event stream itself broke down (a scan
/// error or premature end of input); a finding has already been recorded in
/// that case. Restricted-schema violations record a finding but still
/// produce a best-effort value so that sibling problems keep surfacing.
fn next_node<T: saphyr_parser::Input>(
    parser: &mut Parser<'_, T>,
    findings: &mut Vec<Finding>,
    path: &str,
    first_line: usize,
) -> Result<serde_json::Value, ()> {
    match parser.next() {
        Some(Ok((event, span))) => build_node(event, span, parser, findings, path, first_line),
        Some(Err(err)) => {
            findings.push(scan_error_finding(path, first_line, &err));
            Err(())
        }
        None => {
            findings.push(Finding::new(path, None, "unexpected end of yaml document"));
            Err(())
        }
    }
}

/// Builds a value from an event already pulled off the parser.
fn build_node<T: saphyr_parser::Input>(
    event: Event<'_>,
    span: Span,
    parser: &mut Parser<'_, T>,
    findings: &mut Vec<Finding>,
    path: &str,
    first_line: usize,
) -> Result<serde_json::Value, ()> {
    let line = file_line(span.start, first_line);
    match event {
        Event::Scalar(value, style, anchor_id, tag) => {
            check_anchor_and_tag(anchor_id, &tag, path, line, findings);
            Ok(resolve_scalar(&value, style, path, line, findings))
        }
        Event::Alias(_) => {
            findings.push(Finding::new(
                path,
                Some(line),
                "aliases are not part of restricted yaml",
            ));
            Ok(serde_json::Value::Null)
        }
        Event::SequenceStart(anchor_id, tag) => {
            check_anchor_and_tag(anchor_id, &tag, path, line, findings);
            let mut items = Vec::new();
            loop {
                match parser.next() {
                    Some(Ok((Event::SequenceEnd, _))) => break,
                    Some(Ok((item_event, item_span))) => {
                        let item =
                            build_node(item_event, item_span, parser, findings, path, first_line)?;
                        items.push(item);
                    }
                    Some(Err(err)) => {
                        findings.push(scan_error_finding(path, first_line, &err));
                        return Err(());
                    }
                    None => {
                        findings.push(Finding::new(path, None, "unexpected end of yaml sequence"));
                        return Err(());
                    }
                }
            }
            Ok(serde_json::Value::Array(items))
        }
        Event::MappingStart(anchor_id, tag) => {
            check_anchor_and_tag(anchor_id, &tag, path, line, findings);
            build_mapping(parser, findings, path, first_line)
        }
        // These events cannot appear where a node is expected; the parser's
        // own structure guarantees this branch is unreachable in practice.
        _ => {
            findings.push(Finding::new(path, Some(line), "unexpected yaml structure"));
            Err(())
        }
    }
}

fn build_mapping<T: saphyr_parser::Input>(
    parser: &mut Parser<'_, T>,
    findings: &mut Vec<Finding>,
    path: &str,
    first_line: usize,
) -> Result<serde_json::Value, ()> {
    let mut map = serde_json::Map::new();
    loop {
        let (key_event, key_span) = match parser.next() {
            Some(Ok((Event::MappingEnd, _))) => break,
            Some(Ok(pair)) => pair,
            Some(Err(err)) => {
                findings.push(scan_error_finding(path, first_line, &err));
                return Err(());
            }
            None => {
                findings.push(Finding::new(path, None, "unexpected end of yaml mapping"));
                return Err(());
            }
        };
        let key_line = file_line(key_span.start, first_line);
        let key = resolve_key(
            key_event, key_span, parser, findings, path, first_line, key_line,
        )?;

        let value = next_node(parser, findings, path, first_line)?;

        if let Some(key) = key {
            if map.contains_key(&key) {
                findings.push(Finding::new(
                    path,
                    Some(key_line),
                    format!("duplicate key {key:?} in mapping"),
                ));
            } else {
                map.insert(key, value);
            }
        }
    }
    Ok(serde_json::Value::Object(map))
}

/// Resolves a mapping key event to a restricted-schema string key.
///
/// Returns `Ok(None)` when the key violates the restricted schema (complex,
/// non-string, merge, aliased, anchored, or tagged); a finding is recorded
/// and the corresponding value node is still consumed by the caller so the
/// event stream stays in sync.
fn resolve_key<T: saphyr_parser::Input>(
    key_event: Event<'_>,
    key_span: Span,
    parser: &mut Parser<'_, T>,
    findings: &mut Vec<Finding>,
    path: &str,
    first_line: usize,
    key_line: usize,
) -> Result<Option<String>, ()> {
    match key_event {
        Event::Scalar(value, style, anchor_id, tag) => {
            check_anchor_and_tag(anchor_id, &tag, path, key_line, findings);
            if style == ScalarStyle::Plain && value.as_ref() == "<<" {
                findings.push(Finding::new(
                    path,
                    Some(key_line),
                    "merge keys are not part of restricted yaml",
                ));
                return Ok(None);
            }
            let resolved = resolve_scalar(&value, style, path, key_line, findings);
            match resolved {
                serde_json::Value::String(s) => Ok(Some(s)),
                other => {
                    findings.push(Finding::new(
                        path,
                        Some(key_line),
                        format!("mapping key must be a string, found {}", kind_of(&other)),
                    ));
                    Ok(None)
                }
            }
        }
        Event::Alias(_) => {
            findings.push(Finding::new(
                path,
                Some(key_line),
                "aliases are not part of restricted yaml",
            ));
            Ok(None)
        }
        Event::SequenceStart(..) | Event::MappingStart(..) => {
            findings.push(Finding::new(
                path,
                Some(key_line),
                "complex keys are not part of restricted yaml",
            ));
            // Consume the complex key structure so parsing of the value
            // that follows stays aligned with the event stream.
            build_node(key_event, key_span, parser, findings, path, first_line)?;
            Ok(None)
        }
        _ => {
            findings.push(Finding::new(
                path,
                Some(key_line),
                "unexpected yaml structure",
            ));
            Err(())
        }
    }
}

fn kind_of(value: &serde_json::Value) -> &'static str {
    match value {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "a boolean",
        serde_json::Value::Number(_) => "a number",
        serde_json::Value::String(_) => "a string",
        serde_json::Value::Array(_) => "a sequence",
        serde_json::Value::Object(_) => "a mapping",
    }
}

fn check_anchor_and_tag(
    anchor_id: usize,
    tag: &Option<Cow<'_, Tag>>,
    path: &str,
    line: usize,
    findings: &mut Vec<Finding>,
) {
    // Valid anchor ids start at 1; 0 means the node was not anchored.
    if anchor_id != 0 {
        findings.push(Finding::new(
            path,
            Some(line),
            "anchors are not part of restricted yaml",
        ));
    }
    if tag.is_some() {
        findings.push(Finding::new(
            path,
            Some(line),
            "explicit tags are not part of restricted yaml",
        ));
    }
}

/// Resolves a scalar under the restricted schema.
///
/// Only plain scalars are candidates for `null`/boolean/number resolution;
/// quoted, literal, and folded scalars are always strings regardless of
/// content. An empty plain scalar is a loading error but still yields a
/// value (empty string) so callers can keep collecting findings.
fn resolve_scalar(
    value: &str,
    style: ScalarStyle,
    path: &str,
    line: usize,
    findings: &mut Vec<Finding>,
) -> serde_json::Value {
    if style != ScalarStyle::Plain {
        return serde_json::Value::String(value.to_string());
    }
    if value.is_empty() {
        findings.push(Finding::new(
            path,
            Some(line),
            "empty plain scalar; quote an intentional empty string",
        ));
        return serde_json::Value::String(String::new());
    }
    match value {
        "null" | "~" => return serde_json::Value::Null,
        "true" => return serde_json::Value::Bool(true),
        "false" => return serde_json::Value::Bool(false),
        _ => {}
    }
    if let Some(has_fraction_or_exponent) = classify_number(value) {
        if !has_fraction_or_exponent && let Ok(int) = value.parse::<i64>() {
            return serde_json::Value::Number(int.into());
        }
        if let Ok(float) = value.parse::<f64>()
            && let Some(number) = serde_json::Number::from_f64(float)
        {
            return serde_json::Value::Number(number);
        }
    }
    serde_json::Value::String(value.to_string())
}

/// Checks a plain scalar against the RFC 8259 number grammar:
/// `^-?(0|[1-9][0-9]*)(\.[0-9]+)?([eE][+-]?[0-9]+)?$`.
///
/// Returns `None` when the scalar does not match. Returns `Some(true)` when
/// it matches and has a fractional part or exponent (so it must resolve as
/// a float), `Some(false)` when it is integral.
fn classify_number(value: &str) -> Option<bool> {
    let bytes = value.as_bytes();
    let mut idx = 0;

    if bytes.first() == Some(&b'-') {
        idx += 1;
    }

    let int_start = idx;
    match bytes.get(idx) {
        Some(b'0') => idx += 1,
        Some(b'1'..=b'9') => {
            idx += 1;
            while matches!(bytes.get(idx), Some(b'0'..=b'9')) {
                idx += 1;
            }
        }
        _ => return None,
    }
    if idx == int_start {
        return None;
    }

    let mut has_fraction_or_exponent = false;

    if bytes.get(idx) == Some(&b'.') {
        idx += 1;
        let frac_start = idx;
        while matches!(bytes.get(idx), Some(b'0'..=b'9')) {
            idx += 1;
        }
        if idx == frac_start {
            return None;
        }
        has_fraction_or_exponent = true;
    }

    if matches!(bytes.get(idx), Some(b'e' | b'E')) {
        idx += 1;
        if matches!(bytes.get(idx), Some(b'+' | b'-')) {
            idx += 1;
        }
        let exp_start = idx;
        while matches!(bytes.get(idx), Some(b'0'..=b'9')) {
            idx += 1;
        }
        if idx == exp_start {
            return None;
        }
        has_fraction_or_exponent = true;
    }

    if idx == bytes.len() {
        Some(has_fraction_or_exponent)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load(source: &str) -> Result<serde_json::Value, Vec<Finding>> {
        load_restricted_yaml(source, "test.yaml", 1)
    }

    #[test]
    fn simple_mapping_with_all_scalar_types() {
        let value = load("title: Hello\ncount: 3\nratio: 1.5\nactive: true\nempty: null\n")
            .expect("valid document");
        assert_eq!(
            value,
            serde_json::json!({
                "title": "Hello",
                "count": 3,
                "ratio": 1.5,
                "active": true,
                "empty": null,
            })
        );
    }

    #[test]
    fn quoted_true_stays_a_string() {
        let value = load("flag: \"true\"\n").expect("valid document");
        assert_eq!(value, serde_json::json!({"flag": "true"}));
    }

    #[test]
    fn number_resolution() {
        let value =
            load("int: 42\nfloat: 1.25\nexp: 1e10\nneg_zero: -0\nleading_zero: \"placeholder\"\n")
                .expect("valid document");
        assert_eq!(value["int"], serde_json::json!(42));
        assert_eq!(value["float"], serde_json::json!(1.25));
        assert_eq!(value["exp"], serde_json::json!(1e10));
        assert_eq!(value["neg_zero"], serde_json::json!(0));
        assert!(value["neg_zero"].is_number());
    }

    #[test]
    fn leading_zero_stays_a_string() {
        let value = load("code: 01\n").expect("valid document");
        assert_eq!(value, serde_json::json!({"code": "01"}));
    }

    #[test]
    fn null_true_false_literals() {
        let value = load("a: null\nb: true\nc: false\n").expect("valid document");
        assert_eq!(value, serde_json::json!({"a": null, "b": true, "c": false}));
    }

    #[test]
    fn duplicate_key_rejected_with_correct_line() {
        let err = load("title: one\nbody: x\ntitle: two\n").unwrap_err();
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].line, Some(3));
        assert_eq!(err[0].message, "duplicate key \"title\" in mapping");
    }

    #[test]
    fn anchor_and_alias_rejected() {
        let err = load("a: &x 1\nb: *x\n").unwrap_err();
        assert!(err.iter().any(|f| f.message.contains("anchors")));
        assert!(err.iter().any(|f| f.message.contains("aliases")));
    }

    #[test]
    fn explicit_tag_rejected() {
        let err = load("a: !!str 1\n").unwrap_err();
        assert!(err.iter().any(|f| f.message.contains("tags")));
    }

    #[test]
    fn merge_key_rejected() {
        let err = load("<<: value\n").unwrap_err();
        assert!(err.iter().any(|f| f.message.contains("merge keys")));
    }

    #[test]
    fn complex_key_rejected() {
        let err = load("? [a, b]\n: value\n").unwrap_err();
        assert!(err.iter().any(|f| f.message.contains("complex keys")));
    }

    #[test]
    fn non_string_key_rejected() {
        let err = load("true: value\n").unwrap_err();
        assert!(err.iter().any(|f| f.message.contains("must be a string")));
    }

    #[test]
    fn empty_plain_scalar_rejected() {
        // An anchor with no scalar text after it is the one syntax that
        // produces a genuinely empty plain scalar event (as opposed to
        // `a:` with a missing value, which the parser fills in as `~`).
        let err = load("a: &x\nb: 1\n").unwrap_err();
        assert!(err.iter().any(|f| f.message.contains("empty plain scalar")));
    }

    #[test]
    fn omitted_value_resolves_to_null() {
        let value = load("a:\nb: 1\n").expect("valid document");
        assert_eq!(value, serde_json::json!({"a": null, "b": 1}));
    }

    #[test]
    fn quoted_tilde_stays_a_string() {
        let value = load("a: \"~\"\n").expect("valid document");
        assert_eq!(value, serde_json::json!({"a": "~"}));
    }

    #[test]
    fn two_documents_rejected() {
        let err = load("a: 1\n---\nb: 2\n").unwrap_err();
        assert!(err.iter().any(|f| f.message.contains("found multiple")));
    }

    #[test]
    fn nested_sequences_and_mappings() {
        let value = load("items:\n  - name: a\n    tags: [x, y]\n  - name: b\n    tags: []\n")
            .expect("valid document");
        assert_eq!(
            value,
            serde_json::json!({
                "items": [
                    {"name": "a", "tags": ["x", "y"]},
                    {"name": "b", "tags": []},
                ]
            })
        );
    }

    #[test]
    fn first_line_offset_arithmetic() {
        // The duplicate key sits on fragment line 2; with first_line 5 it
        // should report file line 6.
        let err = load_restricted_yaml("a: 1\na: 2\n", "test.yaml", 5).unwrap_err();
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].line, Some(6));
    }

    #[test]
    fn scan_error_produces_a_finding() {
        let err = load("key: [1, 2]]\n").unwrap_err();
        assert_eq!(err.len(), 1);
        assert!(err[0].line.is_some());
    }
}
