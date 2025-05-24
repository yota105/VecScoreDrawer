mod data;
mod parser;
mod processor;
mod score;

use clap::Parser;
use parser::parse_score;
use processor::process_score;
use score::generator::generate_score_def_yaml_from_score;

mod cli;
use cli::{Cli, Commands};

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
                        println!("Write file: {}", output_path);
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
        Commands::GenerateScore => {
            let vsc_input = "sample.vsc";
            let yaml_output = "score_workspace/score_def/score_def.yaml";
            let pvsc_output = "score_workspace/parsed_vsc.pvsc";
            let input = match std::fs::read_to_string(std::path::Path::new(vsc_input)) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading file {}: {}", vsc_input, e);
                    return;
                }
            };
            match parse_score(&input) {
                Ok(score) => {
                    let processed_score = process_score(score);

                    // pvsc出力
                    let formatted = format!("Parsed Score:\n{:#?}", processed_score);
                    if let Some(parent) = std::path::Path::new(pvsc_output).parent() {
                        std::fs::create_dir_all(parent).ok();
                    }
                    if let Err(e) = std::fs::write(pvsc_output, &formatted) {
                        eprintln!("Error writing to file {}: {}", pvsc_output, e);
                    } else {
                        println!("Write file: {}", pvsc_output);
                    }

                    // score_def.yaml出力
                    match generate_score_def_yaml_from_score(&processed_score) {
                        Ok(yaml) => {
                            if let Some(parent) = std::path::Path::new(yaml_output).parent() {
                                std::fs::create_dir_all(parent).ok();
                            }
                            if let Err(e) = std::fs::write(yaml_output, &yaml) {
                                eprintln!("Error writing to file {}: {}", yaml_output, e);
                            } else {
                                println!("Generated score_def.yaml: {}", yaml_output);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error generating score_def.yaml: {}", e);
                        }
                    }
                }
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
