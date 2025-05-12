mod data;
mod parser;
mod processor;

use clap::{Parser, Subcommand};
use parser::parse_score;
use processor::process_score;
use std::fs::{read_to_string, write};
use std::path::Path;

#[derive(Parser)]
#[command(name = "VecScoreDrawer")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print the score to standard output
    Print {
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Print { input } => {
            let input = match std::fs::read_to_string(std::path::Path::new(input)) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading file {}: {}", input, e);
                    return;
                }
            };
            match parse_score(&input) {
                Ok(score) => {
                    let processed_score = process_score(score);
                    let formatted = format!("Parsed Score:\n{:#?}", processed_score);
                    println!("{}", formatted);
                },
                Err(errs) => {
                    eprintln!("Parse error(s):");
                    for err in errs {
                        eprintln!("  {}", err);
                    }
                }
            }
        }
    }
}
