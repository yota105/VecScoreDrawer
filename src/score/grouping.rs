// 音価グルーピング・タイ分解ロジック
use crate::data::ScoreElement;
use crate::score::score_def_data::{NoteEntry, NoteAttributes};


/// 小節ごとのScoreElement列からグルーピング後のNoteEntry列を生成
pub fn group_measure_elements(
    measure_num: usize,
    elements: &[ScoreElement],
) -> Vec<NoteEntry> {
    // TODO: タイ・音価グルーピングアルゴリズム実装
    // 仮実装: 各Event/Chord/Tieを1音符としてid採番
    let mut notes = Vec::new();
    let mut id = 1;
    for elem in elements {
        match elem {
            ScoreElement::Event(_ev) => {
                notes.push(NoteEntry {
                    measure: measure_num,
                    id,
                    attributes: vec![NoteAttributes {
                        accidental: "None".to_string(),
                    }],
                });
                id += 1;
            }
            ScoreElement::Chord(_chord) => {
                notes.push(NoteEntry {
                    measure: measure_num,
                    id,
                    attributes: vec![NoteAttributes {
                        accidental: "None".to_string(),
                    }],
                });
                id += 1;
            }
            ScoreElement::Tie(_tie) => {
                notes.push(NoteEntry {
                    measure: measure_num,
                    id,
                    attributes: vec![NoteAttributes {
                        accidental: "None".to_string(),
                    }],
                });
                id += 1;
            }
            ScoreElement::Subdivision(sub) => {
                // 再帰的に分解
                let sub_notes = group_measure_elements(measure_num, &sub.elements);
                for n in sub_notes {
                    notes.push(n);
                    id += 1;
                }
            }
        }
    }
    notes
}
