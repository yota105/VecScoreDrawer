use crate::data::{
    Score, Measure, Beat, ScoreElement, Event, EventType, Subdivision, Chord, Pitch, Tie,
};

/// Represents a parsing error with an optional line number.
#[derive(Debug, Clone)]
pub struct ParseError {
    /// The error message.
    pub message: String,
    /// The 0-based line index where the error occurred, if available.
    pub line: Option<usize>,
}

// Implement Display for easy printing of errors.
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(line) = self.line {
            // Add 1 to line for 1-based display
            write!(f, "Line {}: {}", line + 1, self.message)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

// Implement standard Error trait.
impl std::error::Error for ParseError {}

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
) -> Result<Vec<ScoreElement>, ParseError> {
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
                    return Err(ParseError { message: "Unmatched '['".into(), line: None });
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
                idx = end;
            }
            "{" => {
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
                    return Err(ParseError { message: "Unmatched '{'".into(), line: None });
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
                        return Err(ParseError { message: "Chord may contain only simple events".into(), line: None });
                    }
                }
                elems.push(ScoreElement::Chord(Chord { events }));
                idx = end;
            }
            tok => {
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
fn parse_token(token: &str, prev: &[ScoreElement]) -> Result<ScoreElement, ParseError> {
    if token == "r" {
        return Ok(ScoreElement::Event(Event {
            event_type: EventType::Rest,
            pitch: None,
            pitch_cents: None,
            tie: false,
            duration: 1.0,
        }));
    }
    if token == "t" {
        // 直前の音符・タイ・和音からpitch/durationを取得
        let mut last_pitch = None;
        let mut last_pitch_cents = None;
        let mut last_duration = 1.0;
        for se in prev.iter().rev() {
            match se {
                ScoreElement::Event(ev) if ev.event_type == EventType::Note => {
                    last_pitch = ev.pitch.clone();
                    last_pitch_cents = ev.pitch_cents;
                    last_duration = ev.duration;
                    break;
                }
                ScoreElement::Tie(tie) => {
                    last_pitch = tie.pitch.clone();
                    last_pitch_cents = tie.pitch_cents;
                    last_duration = tie.duration;
                    break;
                }
                ScoreElement::Chord(chord) => {
                    // 和音の場合は最初の音を参照（必要に応じて拡張）
                    if let Some(ev) = chord.events.first() {
                        last_pitch = ev.pitch.clone();
                        last_pitch_cents = ev.pitch_cents;
                        last_duration = ev.duration;
                        break;
                    }
                }
                _ => {}
            }
        }
        return Ok(ScoreElement::Tie(Tie {
            pitch: last_pitch,
            pitch_cents: last_pitch_cents,
            duration: last_duration,
        }));
    }
    let tie_flag = token.ends_with('-');
    let core = token.trim_end_matches('-');
    let pitch = core.parse::<Pitch>()
        .map_err(|e| ParseError {
            message: format!("Invalid pitch `{}`: {}", core, e),
            line: None,
        })?;
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

/// Remove comments from the whole input string.
/// Supports both // (line) and /* ... */ (block, possibly multi-line) comments.
/// Preserves newline characters to maintain line structure.
fn remove_comments_multiline(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    let mut in_block = false;

    while let Some(c) = chars.next() {
        if !in_block && c == '/' {
            if let Some(&next) = chars.peek() {
                if next == '/' {
                    for nc in chars.by_ref() {
                        if nc == '\n' {
                            result.push('\n');
                            break;
                        }
                    }
                    continue;
                } else if next == '*' {
                    in_block = true;
                    chars.next();
                    continue;
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        } else if in_block && c == '*' {
            if let Some(&next) = chars.peek() {
                if next == '/' {
                    in_block = false;
                    chars.next();
                    continue;
                }
            }
        } else if !in_block {
            result.push(c);
        } else if c == '\n' {
            result.push('\n');
        }
    }
    result
}

/// テキスト入力をトークナイズ→パースして Score に変換します。
/// 複数エラーをVec<ParseError>で返す
pub fn parse_score(input: &str) -> Result<Score, Vec<ParseError>> {
    let cleaned_input = remove_comments_multiline(input);
    let mut measures = Vec::new();
    let mut current_meter: Option<(usize, usize)> = None;
    let mut errors = Vec::new();

    for (line_idx, line_content) in cleaned_input.lines().enumerate() {
        let line = line_content.trim();
        if line.is_empty() {
            continue;
        }

        let mut line_errors = Vec::new();
        let mut measure_no = 0;
        let mut line_after_measure_no = "";
        // measure_no, line_after_measure_no の取得
        match line.find(':') {
            Some(idx) => {
                let num_str = line[..idx].trim();
                if num_str.is_empty() {
                    line_errors.push(ParseError {
                        message: "Measure number is missing before ':'".to_string(),
                        line: Some(line_idx),
                    });
                } else {
                    match num_str.parse::<usize>() {
                        Ok(num) => {
                            measure_no = num;
                            line_after_measure_no = line[idx + 1..].trim();
                        },
                        Err(_) => {
                            line_errors.push(ParseError {
                                message: format!("Invalid measure number '{}'", num_str),
                                line: Some(line_idx),
                            });
                        }
                    }
                }
            }
            None => {
                line_errors.push(ParseError {
                    message: "Measure number separator ':' is missing".to_string(),
                    line: Some(line_idx),
                });
            }
        }
        if !line_errors.is_empty() {
            errors.extend(line_errors);
            continue;
        }

        let (meter, content) = if let Some((meter_part, rest)) = line_after_measure_no.split_once(' ') {
            if let Some((num, denom)) = parse_meter(meter_part) {
                (Some((num, denom)), rest.trim())
            } else {
                (None, line_after_measure_no)
            }
        } else {
            (None, line_after_measure_no)
        };

        if current_meter.is_none() && meter.is_none() && measures.is_empty() {
            errors.push(ParseError {
                message: format!("No meter specified in the first measure (Measure {})", measure_no),
                line: Some(line_idx),
            });
            continue;
        } else if current_meter.is_none() && meter.is_none() && !measures.is_empty() {
            // 何もしない
        } else if let Some(m) = meter {
            current_meter = Some(m);
        }

        if content.is_empty() {
            if meter.is_some() { continue; }
            errors.push(ParseError {
                message: format!("No content found after measure number/meter (Measure {})", measure_no),
                line: Some(line_idx),
            });
            continue;
        }

        let current_meter_val = match current_meter {
            Some(m) => m,
            None => {
                errors.push(ParseError {
                    message: format!("Internal error: Meter not set (Measure {})", measure_no),
                    line: Some(line_idx),
                });
                continue;
            }
        };

        let start = match content.find('[') {
            Some(s) => s,
            None => {
                errors.push(ParseError {
                    message: format!("Missing '[' in content '{}' (Measure {})", content, measure_no),
                    line: Some(line_idx),
                });
                continue;
            }
        };
        let end = match content.rfind(']') {
            Some(e) => e,
            None => {
                errors.push(ParseError {
                    message: format!("Missing ']' (Measure {})", measure_no),
                    line: Some(line_idx),
                });
                continue;
            }
        };
        let inner = &content[start + 1..end];
        let tokens = tokenize(inner);

        let mut beats = Vec::new();
        let mut beat_tokens = Vec::new();
        let mut depth = 0;
        let mut beat_errors = Vec::new();
        for token in tokens {
            match token.as_str() {
                "[" | "{" => { depth += 1; beat_tokens.push(token); }
                "]" | "}" => { depth -= 1; beat_tokens.push(token); }
                "," if depth == 0 => {
                    if !beat_tokens.is_empty() {
                        match parse_tokens(&beat_tokens, &[]) {
                            Ok(elements) => beats.push(Beat { elements }),
                            Err(mut e) => {
                                e.line = e.line.or(Some(line_idx));
                                beat_errors.push(e);
                            }
                        }
                        beat_tokens.clear();
                    }
                }
                _ => { beat_tokens.push(token); }
            }
        }
        if !beat_tokens.is_empty() {
            match parse_tokens(&beat_tokens, &[]) {
                Ok(elements) => beats.push(Beat { elements }),
                Err(mut e) => {
                    e.line = e.line.or(Some(line_idx));
                    beat_errors.push(e);
                }
            }
        }
        if !beat_errors.is_empty() {
            errors.extend(beat_errors);
        }

        // beats数のエラーは他のエラーと独立して追加
        if beats.len() != current_meter_val.0 {
            errors.push(ParseError {
                message: format!(
                    "Number of beats ({}) does not match meter ({}) (Measure {})",
                    beats.len(), current_meter_val.0, measure_no
                ),
                line: Some(line_idx),
            });
        }

        // beatsが空でもmeasures.pushはしない（ただし他のエラーは収集）
        if !beats.is_empty() {
            measures.push(Measure {
                number: measure_no,
                beats,
                duration: 0.0,
                unit_duration: 0.0,
                meter: current_meter_val,
            });
        }
    }
    if errors.is_empty() {
        Ok(Score { measures })
    } else {
        Err(errors)
    }
}

/// "4/4" のような文字列を (4,4) に変換
fn parse_meter(s: &str) -> Option<(usize, usize)> {
    let mut parts = s.split('/');
    let num = parts.next()?.parse().ok()?;
    let denom = parts.next()?.parse().ok()?;
    Some((num, denom))
}
