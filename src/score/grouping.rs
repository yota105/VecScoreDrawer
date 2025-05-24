// 音価グルーピング・タイ分解ロジック
use crate::data::{ScoreElement, Event, Tie, Chord, EventType};
use crate::score::score_def_data::{NoteEntry, NoteAttributes};
use num_rational::Ratio;

/// 小節ごとのScoreElement列からグルーピング後のNoteEntry列を生成
pub fn group_measure_elements(
    measure_num: usize,
    elements: &[ScoreElement],
) -> Vec<NoteEntry> {
    // 1. Subdivisionを再帰的に展開し、フラットなイベント列にする
    let mut flat_events = Vec::new();
    flatten_elements(elements, &mut flat_events);

    // 2. タイ・ランを検出し、連続するEvent(tie=true)とTieをまとめる
    let mut notes = Vec::new();
    let mut i = 0;
    while i < flat_events.len() {
        // ランの開始
        let mut total_duration = Ratio::new(0, 1);
        let mut j = i;
        let mut first = true;
        let mut current_event_type = EventType::Note;
        let mut representative_id: Option<usize> = None;
        while j < flat_events.len() {
            match &flat_events[j] {
                FlatElem::Event(ev) => {
                    if first {
                        current_event_type = ev.event_type.clone();
                        representative_id = ev.id.map(|id| id as usize);
                    }
                    if first || (ev.tie && !matches!(ev.event_type, EventType::Rest)) {
                        total_duration += ev.duration.clone();
                        first = false;
                        j += 1;
                    } else {
                        break;
                    }
                }
                FlatElem::Tie(tie) => {
                    total_duration += tie.duration.clone();
                    j += 1;
                }
                FlatElem::Chord(chord) => {
                    // 和音は単独で扱うが、直後にTie(t)があれば和音全体にタイをかける
                    if first {
                        total_duration += chord.events.get(0).map(|e| e.duration.clone()).unwrap_or(Ratio::new(0,1));
                        current_event_type = EventType::Note;
                        representative_id = chord.id.map(|id| id as usize);
                        // 直後にTieがあればdurationを加算
                        if j+1 < flat_events.len() {
                            if let FlatElem::Tie(tie) = &flat_events[j+1] {
                                total_duration += tie.duration.clone();
                                j += 1; // Tieも消費
                            }
                        }
                        j += 1;
                    }
                    break;
                }
            }
        }
        // 3. 記譜値集合Dによる貪欲分解
        let mut remain = total_duration.clone();
        let durations = get_note_durations();
        let mut first_note = true;
        while remain > Ratio::new(0,1) {
            let mut found = false;
            for d in &durations {
                if *d <= remain {
                    notes.push(NoteEntry {
                        measure: measure_num,
                        id: representative_id.unwrap_or(0), // pvscのidを使う
                        attributes: vec![NoteAttributes {
                            r#type: if current_event_type == EventType::Rest { "rest".to_string() } else { "note".to_string() },
                            accidental: "None".to_string(),
                            duration: format!("{}/{}", d.numer(), d.denom()),
                        }],
                        source_ids: if let Some(FlatElem::Chord(chord)) = flat_events.get(i) {
                            Some(chord.events.iter().filter_map(|e| e.id.map(|id| id as usize)).collect())
                        } else {
                            None
                        },
                    });
                    // もし分割音価が複数に分かれる場合、idは最初のidを使い続ける（必要ならsource_idsリスト化も検討）
                    remain -= *d;
                    found = true;
                    break;
                }
            }
            if !found {
                // 分解できない場合はbreak
                break;
            }
        }
        if j == i {
            // 進まなかった場合（例: 単独のChord等）
            j += 1;
        }
        i = j;
    }
    notes
}

/// 記譜値集合D（全音符、2分音符、4分音符、8分音符、16分音符、付点2分音符、付点4分音符など）
fn get_note_durations() -> Vec<Ratio<i32>> {
    vec![
        Ratio::new(4,1), // 全音符
        Ratio::new(3,1), // 付点2分
        Ratio::new(2,1), // 2分
        Ratio::new(3,2), // 付点4分
        Ratio::new(1,1), // 4分
        Ratio::new(3,4), // 付点8分
        Ratio::new(1,2), // 8分
        Ratio::new(1,4), // 16分
    ]
}

/// フラットなイベント列を作る
enum FlatElem<'a> {
    Event(&'a Event),
    Tie(&'a Tie),
    Chord(&'a Chord),
}

fn flatten_elements<'a>(elements: &'a [ScoreElement], out: &mut Vec<FlatElem<'a>>) {
    for elem in elements {
        match elem {
            ScoreElement::Event(ev) => out.push(FlatElem::Event(ev)),
            ScoreElement::Tie(tie) => out.push(FlatElem::Tie(tie)),
            ScoreElement::Chord(chord) => out.push(FlatElem::Chord(chord)),
            ScoreElement::Subdivision(sub) => flatten_elements(&sub.elements, out),
        }
    }
}
