//! Project Template Manager (CLI)

use clap::{Parser, Subcommand, ValueHint};
use lib::commands::*;

/// Cli struct
#[derive(Parser)]
#[clap(name = "Project Template Manager", version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

/// Subcommands enum
#[derive(Subcommand)]
enum Commands {
    /// Select a template for use
    Use {
        /// Template ID
        #[clap(value_parser)]
        template: String,

        /// Path to extract
        #[clap(value_parser, value_hint = ValueHint::DirPath)]
        path: Option<String>,

        /// Initialize with git repository
        #[clap(short, long, action)]
        git: bool,
    },

    /// Lists avaible templates
    List,
}

/// Main function
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Use {
            template,
            path,
            git,
        } => use_cmd(template, path, git),
        Commands::List => list_cmd(),
    }
}
