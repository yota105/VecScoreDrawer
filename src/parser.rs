use crate::data::{
    Score, Measure, Beat, ScoreElement, Event, EventType, Subdivision, Chord, Pitch,
};

/// 1文字ずつ走査し、区切り文字 `[]{} ,` を独立したトークンとして抽出します。
/// 空白はすべてスキップし、その他の文字は一続きのバッファとしてまとめます。
fn tokenize(s: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    for ch in s.chars() {
        match ch {
            '[' | ']' | '{' | '}' | ',' => {
                if !buf.is_empty() {
                    tokens.push(buf.clone());
                    buf.clear();
                }
                tokens.push(ch.to_string());
            }
            c if c.is_whitespace() => {
                // 空白は無視
            }
            _ => buf.push(ch),
        }
    }
    if !buf.is_empty() {
        tokens.push(buf);
    }
    tokens
}

/// トークン列を再帰的にパースして ScoreElement のベクタを返します。
/// `outer_prev` はこのレベルの前にあった要素（tie の解決に利用）。
fn parse_tokens(
    tokens: &[String],
    outer_prev: &[ScoreElement],
) -> Result<Vec<ScoreElement>, String> {
    let mut elems = Vec::new();
    let mut idx = 0;
    while idx < tokens.len() {
        match tokens[idx].as_str() {
            "," => {
                idx += 1;
            }
            "[" => {
                let mut depth = 1;
                let start = idx + 1;
                let mut end = start;
                while end < tokens.len() && depth > 0 {
                    match tokens[end].as_str() {
                        "[" => depth += 1,
                        "]" => depth -= 1,
                        _ => {}
                    }
                    end += 1;
                }
                if depth != 0 {
                    return Err("Unmatched '['".into());
                }
                let inner = &tokens[start..end - 1];
                let mut combined = outer_prev.to_vec();
                combined.extend(elems.clone());
                let sub_elems = parse_tokens(inner, &combined)?;
                let base_division = sub_elems.len() as u32;
                elems.push(ScoreElement::Subdivision(Subdivision {
                    elements: sub_elems,
                    base_division,
                }));
// 対応する '}' を探す
                idx = end;
            }
            "{" => {
// 対応する '}' を探す
                let mut depth = 1;
                let start = idx + 1;
                let mut end = start;
                while end < tokens.len() && depth > 0 {
                    match tokens[end].as_str() {
                        "{" => depth += 1,
                        "}" => depth -= 1,
                        _ => {}
                    }
                    end += 1;
                }
                if depth != 0 {
                    return Err("Unmatched '{'".into());
                }
                let inner = &tokens[start..end - 1];
                let mut combined = outer_prev.to_vec();
                combined.extend(elems.clone());
                let chord_elems = parse_tokens(inner, &combined)?;
                let mut events = Vec::new();
                for se in chord_elems {
                    if let ScoreElement::Event(ev) = se {
                        events.push(ev);
                    } else {
                        return Err("Chord may contain only simple events".into());
                    }
                }
                elems.push(ScoreElement::Chord(Chord { events }));
                idx = end;
            }
            tok => {
// "r", "t", "72-", "C#4" など
                let mut combined = outer_prev.to_vec();
                combined.extend(elems.clone());
                let se = parse_token(tok, &combined)?;
                elems.push(se);
                idx += 1;
            }
        }
    }
    Ok(elems)
}

/// 単一トークンの解釈。tie("t"), rest("r"), note などを処理。
fn parse_token(token: &str, prev: &[ScoreElement]) -> Result<ScoreElement, String> {
// Rest
    if token == "r" {
        return Ok(ScoreElement::Event(Event {
            event_type: EventType::Rest,
            pitch: None,
            pitch_cents: None,
            tie: false,
            duration: 1.0,
        }));
    }
// Tie continuation
    if token == "t" {
        return Ok(ScoreElement::Tie);
    }
// Note (with optional trailing '-')
    let tie_flag = token.ends_with('-');
    let core = token.trim_end_matches('-');
    let pitch = core.parse::<Pitch>()
        .map_err(|e| format!("Invalid pitch `{}`: {}", core, e))?;
// pitch_cents = MIDI note number * 100
    let pitch_cents = match pitch.to_midi_number() {
        Ok(n) => Some((n as u16) * 100),
        Err(_) => None,
    };
    Ok(ScoreElement::Event(Event {
        event_type: EventType::Note,
        pitch: Some(pitch),
        pitch_cents,
        tie: tie_flag,
        duration: 1.0,
    }))
}

/// テキスト入力をトークナイズ→パースして Score に変換します。
pub fn parse_score(input: &str) -> Result<Score, String> {
    let mut measures = Vec::new();
    let mut current_meter: Option<(usize, usize)> = None;

    for (line_idx, line) in input.lines().enumerate() {
        let line_no = line_idx + 1;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Remove measure number (e.g., "1:")
        let line = if let Some(idx) = line.find(':') {
            line[idx + 1..].trim()
        } else {
            line
        };

        // Detect meter
        let (meter, content) = if let Some((meter_part, rest)) = line.split_once(' ') {
            if let Some((num, denom)) = parse_meter(meter_part) {
                (Some((num, denom)), rest.trim())
            } else {
                (None, line)
            }
        } else {
            (None, line)
        };

        // Error if no meter in the first measure
        if current_meter.is_none() && meter.is_none() {
            return Err(format!("Line {}: No meter specified in the first measure", line_no));
        }
        // Update meter if changed
        if let Some(m) = meter {
            current_meter = Some(m);
        }
        let meter = current_meter.expect("meter must be set");

        let start = content.find('[')
            .ok_or_else(|| format!("Line {}: missing '['", line_no))?;
        let end = content.rfind(']')
            .ok_or_else(|| format!("Line {}: missing ']'", line_no))?;
        let inner = &content[start + 1..end];
        let tokens = tokenize(inner);

        // Split and parse by beat
        let mut beats = Vec::new();
        let mut beat_tokens = Vec::new();
        let mut depth = 0;
        for token in tokens {
            match token.as_str() {
                "[" | "{" => {
                    depth += 1;
                    beat_tokens.push(token);
                }
                "]" | "}" => {
                    depth -= 1;
                    beat_tokens.push(token);
                }
                "," if depth == 0 => {
                    // Top-level comma as beat separator
                    beats.push(Beat {
                        elements: parse_tokens(&beat_tokens, &[]).map_err(|e| format!("Line {}: {}", line_no, e))?,
                    });
                    beat_tokens.clear();
                }
                _ => {
                    beat_tokens.push(token);
                }
            }
        }
        // Last beat
        if !beat_tokens.is_empty() {
            beats.push(Beat {
                elements: parse_tokens(&beat_tokens, &[]).map_err(|e| format!("Line {}: {}", line_no, e))?,
            });
        }

        // Beat count check
        if beats.len() != meter.0 {
            return Err(format!(
                "Line {}: Number of beats does not match meter (expected {}, got {})",
                line_no, meter.0, beats.len()
            ));
        }

        measures.push(Measure { beats, meter });
    }
    Ok(Score { measures })
}

/// "4/4" のような文字列を (4,4) に変換
fn parse_meter(s: &str) -> Option<(usize, usize)> {
    let mut parts = s.split('/');
    let num = parts.next()?.parse().ok()?;
    let denom = parts.next()?.parse().ok()?;
    Some((num, denom))
}
