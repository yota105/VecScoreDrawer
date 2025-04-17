// src/main.rs (test driver)
mod data;
mod parser;

use parser::parse_score;

fn main() {
    let input = r#"
1: [72-, t, 76, 79]
2: [71-, [t, [72, 74]], 72, r]
3: [81-, t, 79, 84]
4: [79, [77, [76, 77]], 76, r]
5: [{72, 76, 79}, r]
"#;

    match parse_score(input) {
        Ok(score) => {
            println!("Parsed Score:\n{:#?}", score);
        }
        Err(err) => {
            eprintln!("Error parsing score: {}", err);
        }
    }
}
