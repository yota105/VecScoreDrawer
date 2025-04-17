use crate::score::data::{
    Score, Measure, Beat, ScoreElement, Event, EventType, Subdivision, Chord, Pitch,
};

/// Span represents the location of a token in the input text.
#[derive(Debug, Clone)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

/// Token enumeration for our simple DSL.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(String),
    NoteName(String),
    LBracket, RBracket,  // '[' , ']'
    LBrace,   RBrace,    // '{' , '}'
    Colon,    Comma,     // ':' , ','
    Dash,     Tie,       // '-' , 't'
    Rest,                // 'r'
}

/// ParseError holds an error message and where it occurred.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl ParseError {
    pub fn new(message: &str, line: usize, column: usize) -> Self {
        ParseError { message: message.to_string(), line, column }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} at {}:{}", self.message, self.line, self.column)
    }
}
impl std::error::Error for ParseError {}

/// Convert input text into a sequence of tokens with position info.
pub fn tokenize(input: &str) -> Vec<(Token, Span)> {
    // TODO: implement the scanner / tokenizer.
    unimplemented!()
}

/// Top‑level parse function: from raw DSL text to Score.
pub fn parse_score(input: &str) -> Result<Score, ParseError> {
    let tokens = tokenize(input);
    parse_tokens(&tokens)
}

fn parse_tokens(tokens: &[(Token, Span)]) -> Result<Score, ParseError> {
    let mut idx = 0;
    let mut measures = Vec::new();
    while idx < tokens.len() {
        measures.push(parse_measure(tokens, &mut idx)?);
    }
    Ok(Score { measures })
}

fn parse_measure(tokens: &[(Token, Span)], idx: &mut usize) -> Result<Measure, ParseError> {
    // TODO: parse Number, Colon, then BeatList → Measure
    unimplemented!()
}

fn parse_beat_list(tokens: &[(Token, Span)], idx: &mut usize) -> Result<Vec<Beat>, ParseError> {
    // TODO: parse '[' Element (',' Element)* ']' → Vec<ScoreElement>
    unimplemented!()
}

fn parse_element(tokens: &[(Token, Span)], idx: &mut usize) -> Result<ScoreElement, ParseError> {
    // TODO: dispatch on Token to parse Event / Subdivision / Chord / rest / tie
    unimplemented!()
}

// Additional helpers: parse_event, parse_subdivision, parse_chord, etc.
