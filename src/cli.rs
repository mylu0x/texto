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
        words: usize,
        
        /// Number of texts to generate (defaults to 1)
        #[arg(short, long, default_value_t = 1)]
        count: usize,
        
        /// Generate texts with words in random order
        #[arg(short, long)]
        random: bool
    }
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Text { words, count, random } => commands::text::run(*words, *count, *random)?
    }
    
    Ok(())
}
