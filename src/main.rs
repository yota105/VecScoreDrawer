use std::env;
use std::fs;
use std::error::Error;

use vec_score_drawer::score;
use vec_score_drawer::render;
use crate::score::parser::parse_score;
use crate::render::layout_and_render;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input.dsl> <output.pdf>", args[0]);
        std::process::exit(1);
    }
    let input_path = &args[1];
    let output_path = &args[2];

    let text = fs::read_to_string(input_path)?;
    let score = parse_score(&text)?;
    layout_and_render(&score, output_path)?;
    Ok(())
}
