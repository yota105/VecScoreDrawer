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
                    // Line comment: skip until newline, but keep the newline
                    for nc in chars.by_ref() { // Changed from while let
                        if nc == '\n' {
                            result.push('\n');
                            break;
                        }
                    }
                    continue; // Move to next character after newline or end of input
                } else if next == '*' {
                    // Start block comment
                    in_block = true;
                    chars.next(); // Consume '*'
                    continue; // Move to next character
                } else {
                    // Not a comment start, just a '/'
                    result.push(c);
                }
            } else {
                // End of input after '/'
                result.push(c);
            }
        } else if in_block && c == '*' {
            if let Some(&next) = chars.peek() {
                if next == '/' {
                    // End block comment
                    in_block = false;
                    chars.next(); // Consume '/'
                    continue; // Move to next character
                }
                // else: Just a '*' within the comment, ignore it
            }
            // else: End of input after '*', ignore it
        } else if !in_block { // Changed from else { if ... }
            // Not in a block comment, keep the character
            result.push(c);
        } else if c == '\n' { // Changed from else if inside else block
            // Inside a block comment, but keep newline for line counting
            result.push('\n');
        }
        // else: Inside block comment, ignore the character (unless newline) - This case is implicitly handled now
    }
    result
}

/// テキスト入力をトークナイズ→パースして Score に変換します。
pub fn parse_score(input: &str) -> Result<Score, String> {
    // Remove comments from the entire input first
    let cleaned_input = remove_comments_multiline(input);

    let mut measures = Vec::new();
    let mut current_meter: Option<(usize, usize)> = None;

    // Iterate over the lines of the cleaned input
    for (line_idx, line_content) in cleaned_input.lines().enumerate() {
        let line_no = line_idx + 1; // Line number in the *cleaned* input
        let line = line_content.trim(); // Trim whitespace from the line itself
        if line.is_empty() {
            continue; // Skip empty lines (were potentially comment-only lines)
        }

        // Extract measure number (e.g., "1:") for error reporting
        let (measure_no, line_after_measure_no) = if let Some(idx) = line.find(':') {
            let num_str = &line[..idx].trim();
            let num = num_str.parse::<usize>().unwrap_or(line_no); // Use line_no as fallback
            (num, line[idx + 1..].trim())
        } else {
            (line_no, line) // Assume line number is measure number if no ':'
        };

        // Detect meter
        let (meter, content) = if let Some((meter_part, rest)) = line_after_measure_no.split_once(' ') {
            if let Some((num, denom)) = parse_meter(meter_part) {
                (Some((num, denom)), rest.trim())
            } else {
                // No valid meter found, treat the whole line as content (without meter)
                (None, line_after_measure_no)
            }
        } else {
            // No space found, treat the whole line as content (without meter)
            (None, line_after_measure_no)
        };

        // Error if no meter in the first *non-empty, non-comment* measure line
        if current_meter.is_none() && meter.is_none() && !measures.is_empty() {
             // Allow meterless lines if meter is already set
        } else if current_meter.is_none() && meter.is_none() {
             return Err(format!(
                 "Line {} (Measure {}): No meter specified in the first measure",
                 line_no, measure_no
             ));
        }

        // Update meter if specified on this line
        if let Some(m) = meter {
            current_meter = Some(m);
        }

        // If content is empty after removing meter, skip (e.g., "1: 4/4")
        if content.is_empty() {
             // If only meter was specified, update and continue
             if meter.is_some() { continue; }
             // Otherwise, it might be an empty measure line, handle as needed or error
             // For now, let's assume content is required if no meter is specified here
             return Err(format!(
                 "Line {} (Measure {}): No content found",
                 line_no, measure_no
             ));
        }


        // Ensure meter is set before proceeding
        let current_meter_val = match current_meter {
            Some(m) => m,
            None => {
                 return Err(format!(
                     "Line {} (Measure {}): Internal error: Meter not set before processing content",
                     line_no, measure_no
                 ));
            }
        };

        let start = content.find('[')
            .ok_or_else(|| format!("Line {} (Measure {}): missing '[' in content '{}'", line_no, measure_no, content))?;
        let end = content.rfind(']')
            .ok_or_else(|| format!("Line {} (Measure {}): missing ']'", line_no, measure_no))?;
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
                    if !beat_tokens.is_empty() {
                        beats.push(Beat {
                            elements: parse_tokens(&beat_tokens, &[]).map_err(|e| format!("Line {} (Measure {}): {}", line_no, measure_no, e))?,
                        });
                        beat_tokens.clear();
                    }
                }
                _ => {
                    beat_tokens.push(token);
                }
            }
        }
        // Last beat
        if !beat_tokens.is_empty() {
            beats.push(Beat {
                elements: parse_tokens(&beat_tokens, &[]).map_err(|e| format!("Line {} (Measure {}): {}", line_no, measure_no, e))?,
            });
        }

        // Beat count check
        if beats.len() != current_meter_val.0 {
            return Err(format!(
                "Line {} (Measure {}): Number of beats does not match meter (expected {}, got {})",
                line_no, measure_no, current_meter_val.0, beats.len()
            ));
        }

        measures.push(Measure { beats, meter: current_meter_val });
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
