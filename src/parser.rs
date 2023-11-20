//!
//! Stuff for parsing dg files
//!

const SEPARATOR: &str = "---";

use thiserror::Error;

// TODO don't wildcard import
// use crate::comptime;
use crate::pages::*;

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
    pages: Vec<Page>,
}

impl DgParser {
    pub fn new() -> Self {
        Self {
            state: ParseState::Idle,
            pages: vec![],
        }
    }

    pub fn parse_page(&mut self, page: &[String]) -> Page {
        let page = Page::from_content(page.join("\n"));

        // TODO check if pageonly/perm/nochange

        page
    }

    pub fn parse(&mut self, data: &str) -> Result<()> {
        let lines = data.lines().peekable();

        // temporary buffer for the current page it's processing
        let mut pagebuf = vec![];

        for line in lines {
            if let ParseState::PostLine = self.state {
                if line == SEPARATOR {
                    self.state = ParseState::Idle;
                    continue;
                }

                // push page to pages and reset page
                let page = self.parse_page(&pagebuf);
                self.pages.push(page);
                pagebuf.clear();

                return Err(ParseError::AfterPostline(line.to_string()));
            }

            // if parsing a message, add it to the result
            // OR stop parsing if empty line
            if let ParseState::Message = self.state {
                if line.is_empty() {
                    self.state = ParseState::Idle;
                } else {
                    pagebuf.push(line.to_string());
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

            match line {
                SEPARATOR => (),
                _ => (),
            }
        }

        Ok(())
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
