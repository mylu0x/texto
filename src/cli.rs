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
    /// Just a test command.
    Hello {
        #[arg(short, long)]
        name: String
    },
    
    /// Generate dummy text.
    Text {}
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Hello { name } => {
            println!("Hello, {}!", name);
        },
        Commands::Text {  } => commands::text::run()?
    }
    
    Ok(())
}
