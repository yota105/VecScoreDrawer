mod data;
mod parser;

use parser::parse_score;
use std::fs::write;

fn main() {
    let input = r#"
1: [C5-, t, 76, 79]
2: [71-, [t, [72, 74]], 72, r]
3: [81-, t, 79, 84]
4: [79, [77, [76, 77]], 76, r]
5: [{72, 76, 79}, r]
"#;

    let output_file = "output.txt";

    match parse_score(input) {
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
