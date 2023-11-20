//!
//! Stuff for parsing dg files
//!

const SEPARATOR: &str = "---";

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("Encountered {0} instead of a separator while in PostLine state")]
    AfterPostline(String),
}

type Result<T> = std::result::Result<T, ParseError>;

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

    pub fn parse(&mut self, data: &str) -> Result<Vec<Vec<String>>> {
        let lines = data.lines().peekable();

        let mut pages = vec![];
        let mut page = vec![];

        for line in lines {
            if let ParseState::PostLine = self.state {
                if line == SEPARATOR {
                    self.state = ParseState::Idle;
                    continue;
                }

                return Err(ParseError::AfterPostline(line.to_string()));
            }

            // if parsing a message, add it to the result
            // OR stop parsing if empty line
            if let ParseState::Message = self.state {
                if line.is_empty() {
                    self.state = ParseState::Idle;
                } else {
                    page.push(line.to_string());
                }

                continue;
            }

            if let ParseState::Idle = self.state {
                self.state = match line.trim() {
                    "" => continue,
                    "###" => ParseState::ComptimeScript,
                    _ => ParseState::Metadata,
                }
            }

            // push page to pages and reset page
            pages.push(page.clone());
            page.clear();

            match line {
                SEPARATOR => (),
                _ => (),
            }
        }

        Ok(pages)
    }
}

#[cfg(test)]
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
        let _parsed = parser.parse(data).unwrap();

        // let expected: Vec<Vec<String>> = vec![];

        // assert_eq!(parsed, expected);
    }
}
