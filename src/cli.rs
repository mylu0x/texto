use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Debug, Parser)]
#[command(version, about = env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {    
    /// Generate dummy text.
    Text {
        /// Number of words to generate (defaults to 16)
        #[arg(short, long, default_value_t = 16)]
        words: usize
    }
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Text { words } => commands::text::run(*words)?
    }
    
    Ok(())
}
