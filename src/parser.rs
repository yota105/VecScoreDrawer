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
                // buf をわざわざ flush しない
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
                // 区切り文字はスキップ
                idx += 1;
            }
            "[" => {
                // 対応する ']' を探す
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
                // start .. end-1 が inner
                let inner = &tokens[start..end - 1];
                let mut combined = outer_prev.to_vec();
                combined.extend(elems.clone());
                let sub_elems = parse_tokens(inner, &combined)?;
                let base_division = sub_elems.len() as u32;
                elems.push(ScoreElement::Subdivision(Subdivision {
                    elements: sub_elems,
                    base_division,
                }));
                idx = end; // ']' の次へ
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
    for (line_idx, line) in input.lines().enumerate() {
        let line_no = line_idx + 1;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let content = if let Some((_, rest)) = line.split_once(':') {
            rest.trim()
        } else {
            line
        };
        let start = content.find('[')
            .ok_or_else(|| format!("Line {}: missing '['", line_no))?;
        let end = content.rfind(']')
            .ok_or_else(|| format!("Line {}: missing ']'", line_no))?;
        let inner = &content[start + 1..end];
        let tokens = tokenize(inner);
        let beat = Beat {
            elements: parse_tokens(&tokens, &[])
                .map_err(|e| format!("Line {}: {}", line_no, e))?,
        };
        measures.push(Measure { beats: vec![beat] });
    }
    Ok(Score { measures })
}
