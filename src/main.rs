use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use serde_json::json;

#[derive(Parser)]
#[command(
    name = "specful",
    version,
    about = "Repository-native software specifications"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate the repository against the Specful profiles.
    Validate {
        /// Repository root; defaults to the current directory.
        root: Option<PathBuf>,
        /// Emit findings as JSON. The shape is unstable.
        #[arg(long)]
        json: bool,
    },
    /// Regenerate the committed navigation views (indexes and catalog).
    Index {
        /// Repository root; defaults to the current directory.
        root: Option<PathBuf>,
        /// Report drift without writing anything.
        #[arg(long)]
        check: bool,
    },
    /// Initialize a Specful repository: configuration and directories.
    Init {
        /// Immutable project key for every identifier, e.g. REV.
        #[arg(long)]
        project_key: String,
        /// Repository root; defaults to the current directory.
        root: Option<PathBuf>,
    },
    /// Create an artifact from its scaffold with the next allocated id.
    New {
        /// Artifact kind to create.
        #[arg(value_enum)]
        kind: NewKindArg,
        /// Artifact title; also derives the filename slug.
        #[arg(long)]
        title: String,
        /// Architectural scope for msrs and msdd modules, e.g. backend/sync.
        #[arg(long)]
        scope: Option<String>,
        /// Repository root; defaults to the current directory.
        #[arg(long)]
        root: Option<PathBuf>,
    },
    /// Show the catalog record for an identifier.
    Show {
        /// Identifier to look up, e.g. OK-MSDD-0001.
        id: String,
        /// Repository root; defaults to the current directory.
        #[arg(long)]
        root: Option<PathBuf>,
    },
    /// Trace requirement-to-design links for an identifier.
    Trace {
        /// Identifier to trace, e.g. OK-MSRS-0001.
        id: String,
        /// Repository root; defaults to the current directory.
        #[arg(long)]
        root: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, clap::ValueEnum)]
enum NewKindArg {
    Adr,
    Msrs,
    Msdd,
}

impl From<NewKindArg> for specful::authoring::NewKind {
    fn from(kind: NewKindArg) -> Self {
        match kind {
            NewKindArg::Adr => Self::Adr,
            NewKindArg::Msrs => Self::Msrs,
            NewKindArg::Msdd => Self::Msdd,
        }
    }
}

/// Resolves the effective repository root for a command: the explicit
/// `--root` (or positional root) when given, otherwise the nearest ancestor
/// of the current directory containing `.specful.yaml`, per the root
/// discovery contract in `docs/configuration.md`. `init` never calls this:
/// it initializes the given or current directory as given, without
/// searching upward.
fn resolve_root(root: Option<PathBuf>) -> Result<PathBuf, ExitCode> {
    if let Some(root) = root {
        return Ok(root);
    }
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(error) => {
            println!(
                "{}",
                specful::diagnostics::Finding::new(
                    ".",
                    None,
                    format!("cannot determine current directory: {error}")
                )
                .render()
            );
            return Err(ExitCode::FAILURE);
        }
    };
    specful::config::discover_root(&cwd).map_err(|finding| {
        println!("{}", finding.render());
        ExitCode::FAILURE
    })
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Validate { root, json } => {
            let root = match resolve_root(root) {
                Ok(root) => root,
                Err(code) => return code,
            };
            let findings = specful::repo::validate_repository(&root);
            if json {
                let listing = json!({
                    "findings": findings.iter().map(specful::diagnostics::Finding::to_json).collect::<Vec<_>>(),
                    "count": findings.len(),
                });
                println!("{listing}");
            } else {
                for finding in &findings {
                    println!("{}", finding.render());
                }
                match findings.len() {
                    0 => println!("valid: no findings"),
                    1 => println!("invalid: 1 finding"),
                    count => println!("invalid: {count} findings"),
                }
            }
            if findings.is_empty() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Command::Init { project_key, root } => {
            let root = root.unwrap_or_else(|| PathBuf::from("."));
            match specful::authoring::init(&root, &project_key) {
                Ok(created) => {
                    for path in created {
                        println!("created {path}");
                    }
                    ExitCode::SUCCESS
                }
                Err(findings) => {
                    for finding in &findings {
                        println!("{}", finding.render());
                    }
                    ExitCode::FAILURE
                }
            }
        }
        Command::New {
            kind,
            title,
            scope,
            root,
        } => {
            let root = match resolve_root(root) {
                Ok(root) => root,
                Err(code) => return code,
            };
            match specful::authoring::new_artifact(&root, kind.into(), scope.as_deref(), &title) {
                Ok(path) => {
                    println!("created {path}");
                    println!("complete the remaining placeholders, then run specful index");
                    ExitCode::SUCCESS
                }
                Err(findings) => {
                    for finding in &findings {
                        println!("{}", finding.render());
                    }
                    ExitCode::FAILURE
                }
            }
        }
        Command::Show { id, root } => {
            let root = root.unwrap_or_else(|| PathBuf::from("."));
            match specful::query::show(&root, &id) {
                Ok(rendered) => {
                    print!("{rendered}");
                    ExitCode::SUCCESS
                }
                Err(findings) => {
                    for finding in &findings {
                        println!("{}", finding.render());
                    }
                    ExitCode::FAILURE
                }
            }
        }
        Command::Trace { id, root } => {
            let root = root.unwrap_or_else(|| PathBuf::from("."));
            match specful::query::trace(&root, &id) {
                Ok(rendered) => {
                    print!("{rendered}");
                    ExitCode::SUCCESS
                }
                Err(findings) => {
                    for finding in &findings {
                        println!("{}", finding.render());
                    }
                    ExitCode::FAILURE
                }
            }
        }
        Command::Index { root, check } => {
            let root = match resolve_root(root) {
                Ok(root) => root,
                Err(code) => return code,
            };
            let findings = specful::index::run_index(&root, check);
            for finding in &findings {
                println!("{}", finding.render());
            }
            if findings.is_empty() {
                if check {
                    println!("generated views are current");
                } else {
                    println!("generated views written");
                }
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
    }
}
