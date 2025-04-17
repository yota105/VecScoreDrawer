use std::str::FromStr;

/// Score represents the entire musical score and holds multiple measures.
#[derive(Debug, Clone)]
pub struct Score {
    pub measures: Vec<Measure>,
}

/// Measure represents a single measure and contains a collection of beats.
#[derive(Debug, Clone)]
pub struct Measure {
    pub beats: Vec<Beat>,
}

/// Beat represents a single beat and stores ScoreElements that lie on a fixed grid.
#[derive(Debug, Clone)]
pub struct Beat {
    pub elements: Vec<ScoreElement>,
}

/// ScoreElement recursively represents the elements within a beat.
#[derive(Debug, Clone)]
pub enum ScoreElement {
    /// Single note or rest.
    Event(Event),
    /// Subdivision of a basic unit (e.g. tuplets).
    Subdivision(Subdivision),
    /// Chord: multiple notes sounding simultaneously.
    Chord(Chord),
}

/// Event represents a single note or a rest.
#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,  // Note or Rest
    pub pitch: Option<Pitch>,   // Some(Pitch) for notes, None for rests
    pub tie: bool,              // tied to the next event?
    pub duration: f32,          // relative to one basic unit (e.g. 1.0)
}

/// Distinguishes notes from rests.
#[derive(Debug, Clone)]
pub enum EventType {
    Note,
    Rest,
}

/// Subdivision is used when subdividing a basic unit further (e.g. triplets).
#[derive(Debug, Clone)]
pub struct Subdivision {
    pub elements: Vec<ScoreElement>, // recursive
    pub base_division: u32,          // e.g. 3 for triplets
}

/// Chord represents a chord of simultaneous notes.
#[derive(Debug, Clone)]
pub struct Chord {
    pub events: Vec<Event>,
}

/// Pitch can be specified by MIDI note number or by a note name (e.g. "C#4").
#[derive(Debug, Clone)]
pub enum Pitch {
    Midi(u8), // e.g. 60 = C4
    NoteName {
        letter: NoteLetter,
        accidental: Option<Accidental>,
        octave: i32,
    },
}

#[derive(Debug, Clone)]
pub enum NoteLetter { A, B, C, D, E, F, G }
#[derive(Debug, Clone)]
pub enum Accidental { Sharp, Flat }

/// Parse a string like "60" or "C#4" into a Pitch.
impl FromStr for Pitch {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try MIDI number first
        if let Ok(num) = s.parse::<u8>() {
            return Ok(Pitch::Midi(num));
        }
        let mut chars = s.chars();
        let letter = chars.next().ok_or("Empty input".to_string())?;
        let note_letter = match letter.to_ascii_uppercase() {
            'A' => NoteLetter::A, 'B' => NoteLetter::B, 'C' => NoteLetter::C,
            'D' => NoteLetter::D, 'E' => NoteLetter::E, 'F' => NoteLetter::F,
            'G' => NoteLetter::G,
            _   => return Err(format!("Invalid note letter: {}", letter)),
        };
        // Optional accidental
        let mut accidental = None;
        let remaining: String;
        if let Some(ch) = chars.next() {
            if ch == '#' || ch == 'b' {
                accidental = Some(if ch == '#' { Accidental::Sharp } else { Accidental::Flat });
                remaining = chars.collect();
            } else {
                remaining = std::iter::once(ch).chain(chars).collect();
            }
        } else {
            return Err("Missing octave information".to_string());
        }
        let octave: i32 = remaining.parse().map_err(|_| "Invalid octave number".to_string())?;
        Ok(Pitch::NoteName { letter: note_letter, accidental, octave })
    }
}
