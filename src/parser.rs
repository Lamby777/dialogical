//!
//! Stuff for parsing dg files
//!

const SEPARATOR: &str = "---";

/// possible states the parser can be in
enum ParseState {
    /// Waiting to start a new interaction or comptime script
    Idle,

    /// Script content
    ComptimeScript,

    /// Stuff before a message
    Metadata,

    // Empty line before a message
    PreLine,

    /// Text content said by a character
    Message,

    // Empty line after message, before the separator
    PostLine,
}

pub struct DgParser {
    state: ParseState,
}

impl DgParser {
    pub fn new() -> Self {
        Self {
            state: ParseState::Idle,
        }
    }

    pub fn parse(&mut self, data: &str) -> Vec<String> {
        let lines = data.lines().peekable();

        let mut res = vec![];

        for line in lines {
            // if parsing a message, add it to the result
            // OR stop parsing if empty line
            if let ParseState::Message = self.state {
                if line.is_empty() {
                    self.state = ParseState::Idle;
                } else {
                    res.push(line.to_string());
                }
            }

            match line {
                SEPARATOR => (),
                _ => (),
            }
        }

        res
    }
}

mod tests {
    use super::*;

    #[test]
    fn parse_many() {
        let data = r#"%Interaction

---
NAME Deez
VOX Deez

When the words are sus

---
NAME Gamer

Words go brrr

---

When the imposter is sus

---
###

// Another Page
Echo hello world

###
---
NAME Siva
VOX Siva

---

"#;

        let mut parser = DgParser::new();
        let _res = parser.parse(data);
    }
}
