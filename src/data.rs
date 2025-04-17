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
    /// When it is a single note or rest (Event)
    Event(Event),
    /// When a unit is subdivided further (e.g., tuplets)
    Subdivision(Subdivision),
    /// In the case of a chord. Multiple events (notes) sound simultaneously.
    Chord(Chord),
}

/// Event represents a single note or rest.
#[derive(Debug, Clone)]
pub struct Event {
    /// Indicates whether it is a note or a rest.
    pub event_type: EventType,
    /// For notes, Some(Pitch) is provided; for rests, it is None.
    pub pitch: Option<Pitch>,
    /// Flag indicating whether it is tied to the following note.
    pub tie: bool,
    /// Relative duration with respect to the basic unit of the beat (typically 1.0 is standard).
    pub duration: f32,
}

/// EventType distinguishes between notes and rests.
#[derive(Debug, Clone)]
pub enum EventType {
    Note,
    Rest,
}

/// Subdivision is used when subdividing a basic unit further (e.g., tuplets).
#[derive(Debug, Clone)]
pub struct Subdivision {
    /// A collection of subdivided elements (recursively storing ScoreElements).
    pub elements: Vec<ScoreElement>,
    /// The subdivision factor (for example, 3 means a triplet).
    pub base_division: u32,
}

/// Chord represents a chord with multiple simultaneous sounding events.
#[derive(Debug, Clone)]
pub struct Chord {
    /// Each note (Event) that makes up the chord (typically all sounding at the same time).
    pub events: Vec<Event>,
}

/// Pitch represents a musical pitch that can be specified either by a MIDI note number or by note name.
#[derive(Debug, Clone)]
pub enum Pitch {
    /// Specified as a MIDI note number (e.g., 60 corresponds to "C4").
    Midi(u8),
    /// Specified by note name. It holds the note letter, an optional accidental (Sharp/Flat),
    /// and an octave number.
    NoteName {
        letter: NoteLetter,
        accidental: Option<Accidental>,
        octave: i32,
    },
}

/// NoteLetter represents the basic letters (A through G) of a note name.
#[derive(Debug, Clone)]
pub enum NoteLetter {
    A, B, C, D, E, F, G,
}

/// Accidental indicates a sharp or flat.
#[derive(Debug, Clone)]
pub enum Accidental {
    Sharp,
    Flat,
}

/// Implements FromStr so that Pitch can be parsed from text (e.g., "60" or "C#4").
impl FromStr for Pitch {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First, try to parse as a u8 for a MIDI note number.
        if let Ok(num) = s.parse::<u8>() {
            return Ok(Pitch::Midi(num));
        }
        let mut chars = s.chars();
        // The first character is interpreted as the note letter (A-G).
        let letter = chars.next().ok_or("Empty input".to_string())?;
        let note_letter = match letter.to_ascii_uppercase() {
            'A' => NoteLetter::A,
            'B' => NoteLetter::B,
            'C' => NoteLetter::C,
            'D' => NoteLetter::D,
            'E' => NoteLetter::E,
            'F' => NoteLetter::F,
            'G' => NoteLetter::G,
            _ => return Err(format!("Invalid note letter: {}", letter)),
        };

        // Determine if the next character is an accidental ('#' or 'b') or the start of the octave number.
        let mut accidental = None;
        let remaining: String;
        if let Some(ch) = chars.next() {
            if ch == '#' || ch == 'b' {
                accidental = Some(if ch == '#' { Accidental::Sharp } else { Accidental::Flat });
                remaining = chars.collect();
            } else {
                // If not an accidental, treat it as the beginning of the octave number.
                remaining = std::iter::once(ch).chain(chars).collect();
            }
        } else {
            return Err("Missing octave information".to_string());
        }
        // Parse the remainder as the octave number.
        let octave: i32 = remaining.parse().map_err(|_| "Invalid octave number".to_string())?;
        Ok(Pitch::NoteName { letter: note_letter, accidental, octave })
    }
}
