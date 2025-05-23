use crate::data::{Score, ScoreElement};
use num_rational::Ratio;

/// ScoreElementのDurationを再帰的に割り当てる（分数で計算）
fn assign_element_durations(elements: &mut [ScoreElement], duration: Ratio<i32>) {
    for elem in elements {
        match elem {
            ScoreElement::Event(ev) => {
                ev.duration = duration;
            }
            ScoreElement::Tie(tie) => {
                tie.duration = duration;
            }
            ScoreElement::Subdivision(sub) => {
                let div = sub.base_division as i32;
                let sub_duration = duration / Ratio::from_integer(div);
                assign_element_durations(&mut sub.elements, sub_duration);
            }
            ScoreElement::Chord(chord) => {
                for ev in &mut chord.events {
                    ev.duration = duration;
                }
            }
        }
    }
}

/// スコア全体を処理する関数（例: タイや持続時間の計算など）
pub fn process_score(mut score: Score) -> Score {
    for part in &mut score.parts {
        for measure in &mut part.measures {
            // meter: (分子, 分母) 例: (4, 4)
            let (beats, beat_type) = measure.meter;
            let beat_length = 4.0 / beat_type as f32;
            measure.duration = beats as f32 * beat_length;
            measure.unit_duration = beat_length;
            // 各拍にunit_durationを割り当てる
            for beat in &mut measure.beats {
                beat.duration = beat_length;
                // Ratio::from_float(beat_length)はRatio<BigInt>を返すため、i32で分数を作る
                let beat_ratio = Ratio::new((beat_length * 10000.0) as i32, 10000);
                assign_element_durations(&mut beat.elements, beat_ratio);
            }
        }
    }
    score
}
