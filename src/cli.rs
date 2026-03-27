use clap::{Parser, Subcommand};

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
    }
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Hello { name } => {
            println!("Hello, {}!", name);
        }
    }
    
    Ok(())
}
