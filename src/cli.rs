use anyhow::Context;
use clap::{Parser, Subcommand};

use crate::commands::scan;

pub fn run() -> Result<(), anyhow::Error> {
    let opt = Cli::parse();

    match opt.command {
        Commands::Scanner(command) => command
            .execute()
            .with_context(|| "Failed to perform scans".to_string()),
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "scan")]
    Scanner(scan::Command),
}

#[derive(Debug, Parser)]
#[command(
    author,
    about = "Soliris",
    long_about = "Soliris is a Solidity code scanner and optimizer. It processes the AST and runs a set of methods - the scanners - to help improve your code!"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}
