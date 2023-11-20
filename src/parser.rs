//!
//! Stuff for parsing dg files
//!

/// possible states the parser can be in
enum ParseState {
    /// Waiting to start a new interaction or comptime script
    Idle,

    /// Stuff before a message
    Metadata,

    /// Text content said by a character
    Message,

    /// Script content
    ComptimeScript,
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

    pub fn parse(&self, data: &str) -> Vec<String> {
        let lines = data.lines();

        let res = vec![];

        for line in lines {
            match line {
                "---" => (),
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

        let parser = DgParser::new();

        let _res = parser.parse(data);
    }
}
