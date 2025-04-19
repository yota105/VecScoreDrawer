mod data;
mod parser;

use parser::parse_score;
use std::fs::{read_to_string, write}; // read_to_string を追加
use std::path::Path; // Path を追加

fn main() {
    let input_file = "sample.vsc"; // 入力ファイル名を指定
    let output_file = "output.txt";

    // ファイルを読み込む
    let input = match read_to_string(Path::new(input_file)) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", input_file, e);
            return; // エラーが発生したら終了
        }
    };

    match parse_score(&input) { // &input を渡すように変更
        Ok(score) => {
            let formatted = format!("Parsed Score:\n{:#?}", score);
            println!("{}", formatted);
            if let Err(e) = write(output_file, &formatted) {
                eprintln!("File write error: {}", e);
            } else {
                println!("Score successfully written to {}", output_file);
            }
        },
        Err(err) => eprintln!("Parse error: {}", err),
    }
}
