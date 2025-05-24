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
        SubCommand::Render(render_args) => {
            // SVGレンダリング処理
            use crate::render::input::load_score_def;
            use crate::score::score_def_data::ScoreDef;
            use svg::node::element::{Group, Line, Circle};
            use svg::Document;

            let yaml_path = "score_workspace/score_def/score_def.yaml";
            let score_def = match load_score_def(yaml_path) {
                Ok(sd) => sd,
                Err(e) => {
                    eprintln!("score_def.yamlの読み込み失敗: {}", e);
                    return;
                }
            };

            // 最初のパート・最初の小節のnotesを抽出
            let part = match score_def.score.parts.get(0) {
                Some(p) => p,
                None => {
                    eprintln!("パートが見つかりません");
                    return;
                }
            };
            let measure_num = 1;
            let notes: Vec<_> = part.notes.iter().filter(|n| n.measure == measure_num).collect();

            // SVGパラメータ
            let width = 600;
            let height = 120;
            let staff_top = 40;
            let staff_left = 50;
            let staff_spacing = 12;
            let staff_lines = 5;
            let note_radius = 7;

            // 5線譜を描画
            let mut group = Group::new();
            for i in 0..staff_lines {
                let y = staff_top + i * staff_spacing;
                group = group.add(Line::new()
                    .set("x1", staff_left)
                    .set("y1", y)
                    .set("x2", width - staff_left)
                    .set("y2", y)
                    .set("stroke", "black")
                    .set("stroke-width", 2));
            }

            // 音符を等間隔で配置
            let note_count = notes.len().max(1);
            let note_spacing = ((width - staff_left * 2) as f32) / (note_count as f32 + 1.0);
            for (i, _note) in notes.iter().enumerate() {
                let cx = staff_left as f32 + note_spacing * (i as f32 + 1.0);
                let cy = staff_top as f32 + staff_spacing as f32 * 2.0; // 仮の高さ（譜表中央）
                group = group.add(Circle::new()
                    .set("cx", cx)
                    .set("cy", cy)
                    .set("r", note_radius)
                    .set("fill", "black"));
            }

            let document = Document::new()
                .set("viewBox", (0, 0, width, height))
                .add(group);

            // SVGファイルに保存
            match svg::save(&render_args.output, &document) {
                Ok(_) => println!("SVGを出力しました: {}", render_args.output),
                Err(e) => eprintln!("SVG出力失敗: {}", e),
            }
        }
    }
}
