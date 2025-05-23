// Instrument section representation
#[derive(Debug)]
pub struct InstrumentSection {
    pub name: String,
    pub measures: Vec<String>,
}

// Parse .vsc text and extract instrument sections
pub fn parse_instruments(input: &str) -> Vec<InstrumentSection> {
    let mut sections = Vec::new();
    let mut current: Option<InstrumentSection> = None;

    for line in input.lines() {
        if let Some(name) = line.strip_prefix("#[Instrument(").and_then(|l| l.strip_suffix(")]")) {
            if let Some(sec) = current.take() {
                sections.push(sec);
            }
            current = Some(InstrumentSection {
                name: name.to_string(),
                measures: Vec::new(),
            });
        } else if let Some(sec) = current.as_mut() {
            if !line.trim().is_empty() {
                sec.measures.push(line.trim().to_string());
            }
        }
    }
    if let Some(sec) = current {
        sections.push(sec);
    }
    sections
}
