//! Answers lookup (`show`) and traceability (`trace`) queries directly from
//! the committed catalog. Both commands are deliberately thin: they never
//! re-parse markdown or re-collect artifacts, and they carry no drift check
//! of their own (`specful validate` owns that).

use std::path::Path;

use crate::diagnostics::Finding;
use crate::index::CATALOG_PATH;

/// One catalog entry, as loosely typed as the JSON it came from. Only the
/// fields queries need are read; unknown fields are ignored.
type Entry = serde_json::Value;

fn load_catalog(root: &Path) -> Result<Vec<Entry>, Vec<Finding>> {
    let content = std::fs::read_to_string(root.join(CATALOG_PATH)).map_err(|_| {
        vec![Finding::new(
            CATALOG_PATH,
            None,
            "missing catalog; run specful index",
        )]
    })?;
    let catalog: serde_json::Value = serde_json::from_str(&content).map_err(|_| {
        vec![Finding::new(
            CATALOG_PATH,
            None,
            "missing catalog; run specful index",
        )]
    })?;
    Ok(catalog["artifacts"].as_array().cloned().unwrap_or_default())
}

fn unknown(id: &str) -> Vec<Finding> {
    vec![Finding::new(
        CATALOG_PATH,
        None,
        format!("unknown identifier {id}"),
    )]
}

/// The identifier kind inferred from its `<PROJECT>-<KIND>-<NNNN>` shape.
fn infer_kind(id: &str) -> Option<&'static str> {
    let segment = id.rsplit('-').nth(1)?;
    match segment {
        "ADR" => Some("adr"),
        "MSRS" => Some("msrs"),
        "MSDD" => Some("msdd"),
        "REQ" => Some("req"),
        _ => None,
    }
}

fn find<'a>(entries: &'a [Entry], id: &str) -> Option<&'a Entry> {
    entries.iter().find(|entry| entry["id"] == id)
}

fn str_field<'a>(entry: &'a Entry, field: &str) -> Option<&'a str> {
    entry[field].as_str()
}

fn array_field(entry: &Entry, field: &str) -> Vec<String> {
    entry[field]
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

/// Finds the MSRS module that declares `req_id` among its `requirements`.
fn owning_module<'a>(entries: &'a [Entry], req_id: &str) -> Option<&'a Entry> {
    entries.iter().find(|entry| {
        entry["kind"] == "msrs"
            && array_field(entry, "requirements")
                .iter()
                .any(|r| r == req_id)
    })
}

/// Answers `specful show <id>`: one line per stored field of the identified
/// artifact, or the owning module for a requirement identifier.
pub fn show(root: &Path, id: &str) -> Result<String, Vec<Finding>> {
    let entries = load_catalog(root)?;
    let Some(kind) = infer_kind(id) else {
        return Err(unknown(id));
    };

    if kind == "req" {
        let Some(module) = owning_module(&entries, id) else {
            return Err(unknown(id));
        };
        let module_id = str_field(module, "id").unwrap_or_default();
        let module_path = str_field(module, "path").unwrap_or_default();
        let mut out = String::new();
        out.push_str(&format!("id: {id}\n"));
        out.push_str("kind: requirement\n");
        out.push_str(&format!("module: {module_id} ({module_path})\n"));
        return Ok(out);
    }

    let Some(entry) = find(&entries, id) else {
        return Err(unknown(id));
    };
    let mut out = String::new();
    out.push_str(&format!("id: {id}\n"));
    out.push_str(&format!("kind: {kind}\n"));
    out.push_str(&format!(
        "title: {}\n",
        str_field(entry, "title").unwrap_or_default()
    ));
    out.push_str(&format!(
        "path: {}\n",
        str_field(entry, "path").unwrap_or_default()
    ));
    if let Some(status) = str_field(entry, "status") {
        out.push_str(&format!("status: {status}\n"));
    }
    for field in [
        "supersedes",
        "superseded-by",
        "governed-by",
        "satisfies",
        "requirements",
    ] {
        let values = array_field(entry, field);
        if !values.is_empty() {
            out.push_str(&format!("{field}: {}\n", values.join(", ")));
        }
    }
    Ok(out)
}

/// Answers `specful trace <id>`: the requirement-to-design traversal for the
/// identified artifact.
pub fn trace(root: &Path, id: &str) -> Result<String, Vec<Finding>> {
    let entries = load_catalog(root)?;
    let Some(kind) = infer_kind(id) else {
        return Err(unknown(id));
    };

    match kind {
        "adr" => {
            if find(&entries, id).is_none() {
                return Err(unknown(id));
            }
            Err(vec![Finding::new(
                CATALOG_PATH,
                None,
                "trace is not defined for ADRs; use specful show",
            )])
        }
        "msrs" => {
            let Some(entry) = find(&entries, id) else {
                return Err(unknown(id));
            };
            let mut out = String::new();
            for req_id in array_field(entry, "requirements") {
                let mut satisfiers: Vec<&str> = entries
                    .iter()
                    .filter(|e| {
                        e["kind"] == "msdd"
                            && array_field(e, "satisfies").iter().any(|s| s == &req_id)
                    })
                    .filter_map(|e| str_field(e, "id"))
                    .collect();
                satisfiers.sort_unstable();
                if satisfiers.is_empty() {
                    out.push_str(&format!("{req_id} <- (untraced)\n"));
                } else {
                    out.push_str(&format!("{req_id} <- {}\n", satisfiers.join(", ")));
                }
            }
            Ok(out)
        }
        "req" => {
            if owning_module(&entries, id).is_none() {
                return Err(unknown(id));
            }
            let mut satisfiers: Vec<(String, String)> = entries
                .iter()
                .filter(|e| {
                    e["kind"] == "msdd" && array_field(e, "satisfies").iter().any(|s| s == id)
                })
                .map(|e| {
                    (
                        str_field(e, "id").unwrap_or_default().to_owned(),
                        str_field(e, "path").unwrap_or_default().to_owned(),
                    )
                })
                .collect();
            satisfiers.sort_unstable();
            if satisfiers.is_empty() {
                Ok("(untraced)\n".to_owned())
            } else {
                let rendered = satisfiers
                    .iter()
                    .map(|(satisfier_id, path)| format!("{satisfier_id} ({path})"))
                    .collect::<Vec<_>>()
                    .join(", ");
                Ok(format!("{rendered}\n"))
            }
        }
        "msdd" => {
            let Some(entry) = find(&entries, id) else {
                return Err(unknown(id));
            };
            let mut satisfies = array_field(entry, "satisfies");
            satisfies.sort_unstable();
            if satisfies.is_empty() {
                return Ok("(no satisfies links)\n".to_owned());
            }
            let mut out = String::new();
            for req_id in satisfies {
                let module_id = owning_module(&entries, &req_id)
                    .and_then(|module| str_field(module, "id"))
                    .unwrap_or("(unknown module)");
                out.push_str(&format!("{req_id} -> {module_id}\n"));
            }
            Ok(out)
        }
        _ => Err(unknown(id)),
    }
}
