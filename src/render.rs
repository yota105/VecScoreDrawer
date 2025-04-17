use crate::score::data::Score;
use std::error::Error;

/// Layout the score and render it to the given output (e.g. PDF, SVG).
pub fn layout_and_render(score: &Score, output_path: &str) -> Result<(), Box<dyn Error>> {
    // 1. Compute page metrics (margins, staff spacing)…
    // 2. Draw staves, notes, rests, chords, tuplets, ties…
    // 3. Export via `printpdf` or `svg` crate.

    unimplemented!()
}
