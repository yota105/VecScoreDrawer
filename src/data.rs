#![allow(dead_code)]
#![allow(clippy::empty_line_after_doc_comments)]

use std::str::FromStr;

/// Score represents the entire musical score and holds multiple measures.
#[derive(Debug, Clone)]
pub struct Score {
    pub measures: Vec<Measure>,
}

/// Measure represents a single measure and contains a collection of beats.
#[derive(Debug, Clone)]
pub struct Measure {
    pub number: usize, // 小節番号を追加
    pub meter: (usize, usize), // (numerator, denominator)
    pub duration: f32,        // 小節全体のDuration（例: 4.0）
    pub unit_duration: f32,   // 小節内の1単位のDuration（例: 1.0）
    pub beats: Vec<Beat>,
}

/// Beat represents a single beat and stores ScoreElements that lie on a fixed grid.
#[derive(Debug, Clone)]
pub struct Beat {
    pub duration: f32, // 拍のDurationを追加
    pub elements: Vec<ScoreElement>,
}

/// ScoreElement recursively represents the elements within a beat.
#[derive(Debug, Clone)]
pub enum ScoreElement {
    Event(Event),          // simple note / rest
    Subdivision(Subdivision),
    Chord(Chord),
    Tie(Tie),             // tie‑continuation marker（音高・Duration付き）
}

/// Tie represents a tie continuation with pitch and duration.
#[derive(Debug, Clone)]
pub struct Tie {
    pub pitch: Option<Pitch>,
    pub pitch_cents: Option<u16>,
    pub duration: num_rational::Ratio<i32>, // 分数で保持
}

/// Event represents a single note or rest.
#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub pitch: Option<Pitch>,
    /// MIDI note number × 100 ( = cents )。rest のときは None
    pub pitch_cents: Option<u16>,
    pub tie: bool,
    pub duration: num_rational::Ratio<i32>, // 分数で保持
}

/// EventType distinguishes between notes and rests.
#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Note,
    Rest,
}

/// Subdivision is used when subdividing a basic unit further (e.g., tuplets).
#[derive(Debug, Clone)]
pub struct Subdivision {
    pub base_division: u32,
    pub elements: Vec<ScoreElement>,
}

/// Chord represents a chord with multiple simultaneous sounding events.
#[derive(Debug, Clone)]
pub struct Chord {
    pub events: Vec<Event>,
}

/// Pitch can be specified either by MIDI note number or by note name.
#[derive(Debug, Clone)]
pub enum Pitch {
    Midi(u8),
    NoteName {
        letter: NoteLetter,
        accidental: Option<Accidental>,
        octave: i32,
    },
}

/// Note letters and accidentals -------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum NoteLetter {
    A, B, C, D, E, F, G,
}

#[derive(Debug, Clone)]
pub enum Accidental {
    Sharp,
    Flat,
}

/// Duration: 整数部と分数部で表現する持続時間
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Duration {
    pub int: i32,         // 整数部分
    pub frac_num: i32,    // 分子（整数のみの場合は0）
    pub frac_den: i32,    // 分母（0不可、整数のみの場合は1）
}

/// --- FromStr ------------------------------------------------------------------------------

impl FromStr for Pitch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // MIDI number?
        if let Ok(num) = s.parse::<u8>() {
            return Ok(Pitch::Midi(num));
        }

        let mut chars = s.chars();
        let letter_char = chars.next().ok_or("Empty input")?;
        let letter = match letter_char.to_ascii_uppercase() {
            'A' => NoteLetter::A,
            'B' => NoteLetter::B,
            'C' => NoteLetter::C,
            'D' => NoteLetter::D,
            'E' => NoteLetter::E,
            'F' => NoteLetter::F,
            'G' => NoteLetter::G,
            _   => return Err(format!("Invalid note letter: {}", letter_char)),
        };

        // accidental?
        let mut accidental = None;
        let rest: String;
        if let Some(ch) = chars.next() {
            if ch == '#' || ch == 'b' {
                accidental = Some(if ch == '#' { Accidental::Sharp } else { Accidental::Flat });
                rest = chars.collect();
            } else {
                rest = std::iter::once(ch).chain(chars).collect();
            }
        } else {
            return Err("Missing octave information".into());
        }

        let octave: i32 = rest.parse().map_err(|_| "Invalid octave".to_string())?;
        Ok(Pitch::NoteName { letter, accidental, octave })
    }
}

/// --- Pitch utility methods (ownership‑safe) -----------------------------------------------

impl Pitch {
    /// Convert to MIDI note number (0–127).
    pub fn midi_number(&self) -> Result<u8, String> {
        match self {
            Pitch::Midi(n) => Ok(*n),
            Pitch::NoteName { letter, accidental, octave } => {
                // semitone offset of the natural letter
                let base = match letter {
                    NoteLetter::C => 0,
                    NoteLetter::D => 2,
                    NoteLetter::E => 4,
                    NoteLetter::F => 5,
                    NoteLetter::G => 7,
                    NoteLetter::A => 9,
                    NoteLetter::B => 11,
                };
                let shift = match accidental {
                    Some(Accidental::Sharp) => 1,
                    Some(Accidental::Flat)  => -1,
                    None                    => 0,
                };
                let midi = (octave + 1) * 12 + base + shift;
                if (0..=127).contains(&midi) {
                    Ok(midi as u8)
                } else {
                    Err(format!("Pitch {:?} out of MIDI range", self))
                }
            }
        }
    }

    #[inline]
    pub fn to_midi_number(&self) -> Result<u8, String> {
        self.midi_number()
    }

    /// Convert to “cents” representation (MIDI × 100).
    pub fn cents(&self) -> Result<u16, String> {
        self.midi_number().map(|n| (n as u16) * 100)
    }
}
