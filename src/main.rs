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
    /// Print the score to standard output or file
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
                    // 出力先を固定
                    let output_path = "score_workspace/parsed_vsc.pvsc";
                    // ディレクトリがなければ作成
                    if let Some(parent) = std::path::Path::new(output_path).parent() {
                        std::fs::create_dir_all(parent).ok();
                    }
                    if let Err(e) = std::fs::write(output_path, &formatted) {
                        eprintln!("Error writing to file {}: {}", output_path, e);
                    } else {
                        println!("書き出しました: {}", output_path);
                    }
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
