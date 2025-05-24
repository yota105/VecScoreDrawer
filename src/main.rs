mod data;
mod parser;
mod processor;
mod score;

use clap::Parser;
use parser::parse_score;
use processor::process_score;
use score::generator::generate_score_def_yaml_from_score;

mod cli;
mod render;
use cli::{Args, SubCommand, RenderArgs};

fn main() {
    let args = Args::parse();
    match &args.subcommand {
        SubCommand::Generate { input, output } => {
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
                    let output_path = output;
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
        SubCommand::GenerateScore => {
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
        SubCommand::Render(render_args) => {
            use crate::render::input::load_score_def;
            use crate::render::backend::svg::render_svg;

            let yaml_path = "score_workspace/score_def/score_def.yaml";
            let pvsc_path = "score_workspace/parsed_vsc.pvsc";
            let score_def = match load_score_def(yaml_path) {
                Ok(sd) => sd,
                Err(e) => {
                    eprintln!("score_def.yamlの読み込み失敗: {}", e);
                    return;
                }
            };
            let pvsc_content = match std::fs::read_to_string(pvsc_path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("pvscファイルの読み込み失敗: {}", e);
                    return;
                }
            };
            match render_svg(&score_def, &pvsc_content, &render_args.output) {
                Ok(_) => println!("SVGを出力しました: {}", render_args.output),
                Err(e) => eprintln!("SVG出力失敗: {}", e),
            }
        }
    }
}
