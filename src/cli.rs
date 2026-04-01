use std::{fs::File, io::{Write, stdin, stdout}, path::PathBuf};

use anyhow::Ok;
use clap::{Parser, Subcommand};

use crate::commands::{self, text::{Format, Lang}, uuid::{Case, UuidVersion}};

#[derive(Debug, Parser)]
#[command(version, about = env!("CARGO_PKG_DESCRIPTION"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Output the result to a file
    #[arg(short, long, global = true)]
    output: Option<PathBuf>,
    
    /// Force overwrite if the output file already exists
    #[arg(long, global = true)]
    force: bool
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
        format: Format,
        
        #[arg(short, long, default_value = " ")]
        separator: String
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
    
    let result: String = match &cli.command {
        Commands::Text { words, count, random, lang, format, separator } => commands::text::run(*words, *count, *random, *lang, *format),
        Commands::Lorem { words } => commands::lorem::run(*words),
        Commands::Uuid { version, count, case } => commands::uuid::run(*version, *count, *case)
    }?;
    
    if let Some(path) = cli.output {        
        if path.exists() && !cli.force { // File already exists AND --force
            print!("File {} already exists. Overwrite? [y/N] ", path.display());
            stdout().flush()?;
            
            let mut choice = String::new();
            stdin().read_line(&mut choice)?;
            
            if !cli.force {
                println!("Hint: You can use --force to force overwriting to a file! (without warnings)")
            }
            
            if choice.trim().to_lowercase() != "y" { // Choice isn't y
                println!("Aborted.");
                return Ok(());
            }
        }
        
        let mut file = File::create(&path)?;
        write!(file, "{}", result)?;
        println!("Saved to {}.", &path.display());
    } else {
        if !result.is_empty() {
            println!("{}", result);
        }
    }
    
    Ok(())
}
