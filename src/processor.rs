use crate::data::Score;

/// スコア全体を処理する関数（例: タイや持続時間の計算など）
pub fn process_score(mut score: Score) -> Score {
    for measure in &mut score.measures {
        // meter: (分子, 分母) 例: (4, 4)
        let (beats, beat_type) = measure.meter;
        let beat_length = 4.0 / beat_type as f32;
        measure.duration = beats as f32 * beat_length;
        measure.unit_duration = beat_length;
    }
    score
}
