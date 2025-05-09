mod data;
mod parser;
mod processor;

use parser::parse_score;
use processor::process_score; // 追加
use std::fs::{read_to_string, write};
use std::path::Path;

fn main() {
    let input_file = "sample.vsc";
    let output_file = "output.txt";

    let input = match read_to_string(Path::new(input_file)) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", input_file, e);
            return;
        }
    };

    match parse_score(&input) {
        Ok(score) => {
            // ここでプロセッサーに渡す
            let processed_score = process_score(score);
            let formatted = format!("Parsed Score:\n{:#?}", processed_score);
            println!("{}", formatted);
            if let Err(e) = write(output_file, &formatted) {
                eprintln!("File write error: {}", e);
            } else {
                println!("Score successfully written to {}", output_file);
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
