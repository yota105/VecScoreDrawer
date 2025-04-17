// src/parser.rs
use crate::data::{
    Score, Measure, Beat, ScoreElement, Event, EventType, Subdivision, Chord, Pitch,
};

/// Parse an entire score from a text in our simple notation.
/// Each non-empty line is treated as one measure containing one beat.
pub fn parse_score(input: &str) -> Result<Score, String> {
    let mut measures = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // strip leading "N:" if present
        let content = if let Some((_, rest)) = line.split_once(':') {
            rest.trim()
        } else {
            line
        };
        // find the outermost [ ... ]
        let start = content.find('[').ok_or("Missing '[' in line")?;
        let end = content.rfind(']').ok_or("Missing ']' in line")?;
        let inner = &content[start + 1..end];
        let elements = parse_elements(inner)?;
        let beat = Beat { elements };
        measures.push(Measure { beats: vec![beat] });
    }
    Ok(Score { measures })
}

/// Split a string like "72-, t, [71, 74], r" into top‑level tokens,
/// then parse each into a ScoreElement.
fn parse_elements(s: &str) -> Result<Vec<ScoreElement>, String> {
    let mut elems = Vec::new();
    let mut buf = String::new();
    let mut depth = 0;
    for c in s.chars() {
        match c {
            '[' | '{' => {
                depth += 1;
                buf.push(c);
            }
            ']' | '}' => {
                depth -= 1;
                buf.push(c);
            }
            ',' if depth == 0 => {
                let token = buf.trim();
                if !token.is_empty() {
                    elems.push(parse_token(token, &elems)?);
                }
                buf.clear();
            }
            _ => buf.push(c),
        }
    }
    if !buf.trim().is_empty() {
        elems.push(parse_token(buf.trim(), &elems)?);
    }
    Ok(elems)
}

/// Parse a single token, using previous elements for tie ("t") handling.
fn parse_token(token: &str, prev: &[ScoreElement]) -> Result<ScoreElement, String> {
    // Rest
    if token == "r" {
        return Ok(ScoreElement::Event(Event {
            event_type: EventType::Rest,
            pitch: None,
            tie: false,
            duration: 1.0,
        }));
    }
    // Tie continuation: duplicate last pitch with tie = true
    if token == "t" {
        if let Some(ScoreElement::Event(last)) = prev.last() {
            if let Some(p) = &last.pitch {
                return Ok(ScoreElement::Event(Event {
                    event_type: EventType::Note,
                    pitch: Some(p.clone()),
                    tie: true,
                    duration: 1.0,
                }));
            }
        }
        return Err("Invalid tie: no preceding note to tie".into());
    }
    // Subdivision: nested [ ... ]
    if token.starts_with('[') && token.ends_with(']') {
        let inner = &token[1..token.len() - 1];
        let sub = parse_elements(inner)?;
        // compute division count before moving `sub`
        let base_division = sub.len() as u32;
        return Ok(ScoreElement::Subdivision(Subdivision {
            elements: sub,
            base_division,
        }));
    }
    // Chord: nested { ... }
    if token.starts_with('{') && token.ends_with('}') {
        let inner = &token[1..token.len() - 1];
        let sub = parse_elements(inner)?;
        let mut events = Vec::new();
        for se in sub {
            if let ScoreElement::Event(ev) = se {
                events.push(ev);
            } else {
                return Err("Chord may contain only simple events".into());
            }
        }
        return Ok(ScoreElement::Chord(Chord { events }));
    }
    // Note (no tie in this simplified parser)
    let core = token.trim_end_matches('-');
    let tie_flag = token.ends_with('-');
    let pitch = core.parse::<Pitch>()
        .map_err(|e| format!("Invalid pitch `{}`: {}", core, e))?;
    Ok(ScoreElement::Event(Event {
        event_type: EventType::Note,
        pitch: Some(pitch),
        tie: tie_flag,
        duration: 1.0,
    }))
}
