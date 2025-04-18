mod data;
mod parser;

use parser::parse_score;
use std::fs::write;

fn main() {
    let input = r#"
1: 4/4 [72-, t, 76, 79]
2: [71-, [t, [72, 74]], 72, r]
// test comment 
3: [81-, t, 79, 84] /* test
test
test */
4: [79, [77, [76, 77]], 76, r]
5: 2/4 [{72, 76, 79}, {72, 77, 81}]
6: 9/8 [{71-, 74-, 79}, t, t, {72, 76, 79}, t, t, r, r, r]
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
