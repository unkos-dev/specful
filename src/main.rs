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
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Validate { root, json } => {
            let root = root.unwrap_or_else(|| PathBuf::from("."));
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
    }
}
