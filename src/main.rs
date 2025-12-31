use anyhow::Result;
use clap::Parser;

use themectl::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.execute()
}
