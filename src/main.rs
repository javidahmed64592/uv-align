//! Update pyproject.toml dependency constraints using versions resolved by uv, with preview and interactive apply support.
//!
//! Usage:
//!
//! uv-bump check                           # Check for dependency updates and show a diff of the changes
//! uv-bump apply [-y] [-i, --interactive]  # Apply dependency updates to pyproject.toml
//! uv-bump update [-y] [-i, --interactive] # Update dependencies and apply changes to pyproject.toml

mod cli;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check => {
            println!("Running: check");
            // TODO: diff logic
        }

        Commands::Apply { yes, interactive } => {
            println!("Running: apply");
            println!("yes={yes}, interactive={interactive}");

            // TODO: apply logic
        }

        Commands::Update { yes, interactive } => {
            println!("Running: update");
            println!("yes={yes}, interactive={interactive}");

            // TODO:
            // 1. uv lock --upgrade
            // 2. compute diff
            // 3. apply changes
        }
    }
}
