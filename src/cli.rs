use clap::{Parser, Subcommand};

use crate::commands::{self, text::{Format, Lang}, uuid::{Case, UuidVersion}};

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
        /// Number of words to generate
        #[arg(short, long, default_value_t = 16)]
        words: usize,
        
        /// Number of texts to generate
        #[arg(short, long, default_value_t = 1)]
        count: usize,
        
        /// Generate texts with words in random order
        #[arg(short, long)]
        random: bool,
        
        /// Language to generate texts
        #[arg(short, long)]
        lang: Option<Lang>,
        
        /// Format to generate
        #[arg(short, long, default_value = "plain")]
        format: Format
    },
    
    /// Generate Lorem Ipsum.
    Lorem {
        /// Number of words to generate
        #[arg(short, long, default_value_t = 19)]
        words: usize
    },
    
    /// Generate UUID.
    Uuid {
        #[arg(short, long, default_value = "v4")]
        version: UuidVersion,
        
        /// Number of uuid to generate
        #[arg(short, long, default_value_t = 1)]
        count: usize,
        
        /// Case to generate (Upper or Lower)
        #[arg(short = 'C', long, default_value = "lower")]
        case: Case
    }
}

pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Text { words, count, random, lang, format } => commands::text::run(*words, *count, *random, *lang, *format)?,
        Commands::Lorem { words } => commands::lorem::run(*words)?,
        Commands::Uuid { version, count, case } => commands::uuid::run(*version, *count, *case)?
    }
    
    Ok(())
}
