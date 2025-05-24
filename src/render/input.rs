use std::fs::File;
use std::io::BufReader;
use anyhow::Result;
use crate::score::score_def_data::ScoreDef;

/// YAMLファイルからScoreDefを読み込む関数
pub fn load_score_def<P: AsRef<std::path::Path>>(path: P) -> Result<ScoreDef> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let score_def: ScoreDef = serde_yaml::from_reader(reader)?;
    Ok(score_def)
}
