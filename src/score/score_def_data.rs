// 楽譜定義YAML(score_def.yaml)用データ構造
use serde::Serialize;

#[derive(Serialize)]
pub struct ScoreDef {
    pub score: ScoreSection,
}

#[derive(Serialize)]
pub struct ScoreSection {
    pub tempo: Vec<TempoSetting>,
    pub key_signature: Vec<KeySignatureSetting>,
    pub parts: Vec<PartSetting>,
}

#[derive(Serialize)]
pub struct TempoSetting {
    pub measure: usize,
    pub position: f32,
    pub bpm: u32,
}

#[derive(Serialize)]
pub struct KeySignatureSetting {
    pub measure: usize,
    pub position: f32,
    pub key: String,
}

#[derive(Serialize)]
pub struct PartSetting {
    pub name: String,
    pub staves: Vec<StavesSetting>,
    pub dynamics: Vec<DynamicsSetting>,
    pub notes: Vec<NoteEntry>,
}

#[derive(Serialize)]
pub struct StavesSetting {
    pub measure: usize,
    pub position: f32,
    pub r#type: String,
    pub lines: Vec<u8>,
}

#[derive(Serialize)]
pub struct DynamicsSetting {
    pub measure: usize,
    pub position: f32,
    pub level: String,
}

#[derive(Serialize)]
pub struct NoteEntry {
    pub measure: usize,
    pub id: usize,
    pub attributes: Vec<NoteAttributes>,
}

#[derive(Serialize)]
pub struct NoteAttributes {
    pub accidental: String,
}
