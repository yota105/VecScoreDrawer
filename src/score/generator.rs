// Score→score_def.yaml変換ロジック
use crate::data::Score;
use crate::score::score_def_data::*;
use crate::score::grouping::group_measure_elements;
use serde_yaml;
use anyhow::Result;

/// Score構造体からScoreDef(YAML用)を生成しYAML文字列として返す
pub fn generate_score_def_yaml_from_score(score: &Score) -> Result<String> {
    // デフォルト値
    let tempo = vec![TempoSetting {
        measure: 1,
        position: 1.0,
        bpm: 120,
    }];
    let key_signature = vec![KeySignatureSetting {
        measure: 1,
        position: 1.0,
        key: "C_Major".to_string(),
    }];

    let mut parts = Vec::new();
    for part in &score.parts {
        let staves = vec![StavesSetting {
            measure: 1,
            position: 1.0,
            r#type: "single".to_string(),
            lines: vec![5],
        }];
        let dynamics = vec![DynamicsSetting {
            measure: 1,
            position: 1.0,
            level: "P".to_string(),
        }];
        let mut notes = Vec::new();
        for measure in &part.measures {
            // 各小節の全BeatのScoreElementをフラット化
            let mut elements = Vec::new();
            for beat in &measure.beats {
                elements.extend(beat.elements.iter().cloned());
            }
            let mut note_entries = group_measure_elements(measure.number, &elements);
            notes.append(&mut note_entries);
        }
        parts.push(PartSetting {
            name: part.name.clone(),
            staves,
            dynamics,
            notes,
        });
    }

    let score_def = ScoreDef {
        score: ScoreSection {
            tempo,
            key_signature,
            parts,
        },
    };

    let yaml = serde_yaml::to_string(&score_def)?;

    // コメント挿入処理
    let mut lines: Vec<String> = Vec::new();
    // Score global definition comment (English)
    lines.push("#------------<Score Global Definition>------------#".to_string());

    let mut in_notes = false;
    let mut last_measure: Option<usize> = None;
    for line in yaml.lines() {
        // notes: セクションに入ったら以降のみmeasureコメントを挿入
        if line.trim_start().starts_with("notes:") {
            in_notes = true;
            lines.push(line.to_string());
            continue;
        }
        if in_notes {
            if let Some(caps) = line.trim().strip_prefix("- measure: ") {
                if let Ok(measure_num) = caps.split_whitespace().next().unwrap_or("").parse::<usize>() {
                    if Some(measure_num) != last_measure {
                        lines.push(format!("#------------<Measure {}>------------#", measure_num));
                        last_measure = Some(measure_num);
                    }
                }
            }
        }
        lines.push(line.to_string());
    }
    Ok(lines.join("\n"))
}
