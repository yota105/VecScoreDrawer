// 楽譜定義YAML(score_def.yaml)用データ構造
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ScoreDef {
    pub score: ScoreSection,
}

#[derive(Serialize, Deserialize)]
pub struct ScoreSection {
    pub tempo: Vec<TempoSetting>,
    pub key_signature: Vec<KeySignatureSetting>,
    pub parts: Vec<PartSetting>,
}

#[derive(Serialize, Deserialize)]
pub struct TempoSetting {
    pub measure: usize,
    pub position: f32,
    pub bpm: u32,
}

#[derive(Serialize, Deserialize)]
pub struct KeySignatureSetting {
    pub measure: usize,
    pub position: f32,
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartSetting {
    pub name: String,
    pub staves: Vec<StavesSetting>,
    pub dynamics: Vec<DynamicsSetting>,
    pub notes: Vec<NoteEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct StavesSetting {
    pub measure: usize,
    pub position: f32,
    pub r#type: String,
    pub lines: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct DynamicsSetting {
    pub measure: usize,
    pub position: f32,
    pub level: String,
}

#[derive(Serialize, Deserialize)]
pub struct NoteEntry {
    pub measure: usize,
    pub id: usize,
    pub attributes: Vec<NoteAttributes>,
    pub source_ids: Option<Vec<usize>>, // 和音の構成音idリスト等
}

#[derive(Serialize, Deserialize)]
pub struct NoteAttributes {
    pub r#type: String,      // "note" or "rest"
    pub accidental: String,  // "None" for rest
    pub duration: String,    // 例: "2/1"
}
