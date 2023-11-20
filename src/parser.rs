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

    #[error("{0} is not a valid interaction ID")]
    InvalidID(String),
}

type Result<T> = std::result::Result<T, ParseError>;

/// possible states the parser can be in
enum ParseState {
    /// Interaction-wide metadata not set yet, we're at the
    /// top of an interaction
    Start,

    /// Waiting to start a new interaction or comptime script
    Idle,

    /// Script content
    ComptimeScript,

    /// Stuff before a message
    Metadata,

    /// Empty line before a message
    PreLine,

    /// Text content said by a character
    Message,

    /// Empty line after message, before the separator
    PostLine,
}

pub struct DgParser {
    state: ParseState,
    pages: Vec<Page>,
    interaction_id: Option<String>,
}

impl DgParser {
    pub fn new() -> Self {
        Self {
            state: ParseState::Start,
            pages: vec![],
            interaction_id: None,
        }
    }

    /// Finish parsing 1 interaction, and clear the state
    /// to prepare for another one.
    ///
    /// `Err` if the parser is in a state where it's not
    /// prepared to finish just yet.
    fn result(&self) -> Result<Interaction> {
        Ok(Interaction {
            id: self
                .interaction_id
                .as_ref()
                .expect("interaction id should not be empty"),
            pages: self.pages.clone(),
        })
    }

    pub fn parse_page(&mut self, page: &[String]) -> Page {
        let page = Page::from_content(page.join("\n"));

        // TODO check if pageonly/perm/nochange

        page
    }

    fn parse_start(&mut self, line: &str) -> Result<()> {
        let line = line.trim();
        if line == "---" || line.is_empty() {
            return Ok(());
        }

        let (percent, id) = line.split_at(1);

        if percent != "%" {
            Err(ParseError::InvalidID(line.to_string()))
        } else {
            self.interaction_id = Some(id.to_owned());
            self.state = ParseState::Idle;
            Ok(())
        }
    }

    fn parse_idle(&mut self, line: &str) {
        self.state = match line.trim() {
            "" => return,
            "###" => ParseState::ComptimeScript,
            _ => ParseState::Metadata,
        }
    }

    fn parse_postline(&mut self, pagebuf: &mut Vec<String>, line: &str) -> Result<()> {
        if line == SEPARATOR {
            self.state = ParseState::Start;
            return Ok(());
        }

        // push page to pages and reset page
        let page = self.parse_page(&pagebuf);
        self.pages.push(page);
        pagebuf.clear();

        Err(ParseError::AfterPostline(line.to_string()))
    }

    fn parse_message(&mut self, pagebuf: &mut Vec<String>, line: &str) {
        // if parsing a message, add it to the result
        // OR stop parsing if empty line
        if line.is_empty() {
            self.state = ParseState::Start;
            return;
        }

        pagebuf.push(line.to_string());
    }

    fn parse_metaline(&mut self, line: &str) {
        if line.is_empty() {
            self.state = ParseState::PreLine;
        }
    }

    pub fn parse(&mut self, data: &str) -> Result<Interaction> {
        let lines = data.lines();

        // temporary buffer for the current page it's processing
        let mut pagebuf = vec![];

        for line in lines {
            use ParseState::*;

            // really wish rustfmt aligned the arrows like in go

            match self.state {
                Start => self.parse_start(line)?,
                Idle => self.parse_idle(line),

                // besides the start, a block can either be
                // a comptime script or a message section
                ComptimeScript => todo!("comptime"),

                Metadata => self.parse_metaline(line),
                PreLine => todo!(),
                Message => self.parse_message(&mut pagebuf, line),
                PostLine => self.parse_postline(&mut pagebuf, line)?,
            }
        }

        self.result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_small_interaction() {
        let data = r#"%Test1

---
NAME Siva
VOX Siva

First page

---
NAME Terra
VOX Terra

Second page
With more words

---

"#;

        let mut parser = DgParser::new();
        let parsed = parser.parse(data).unwrap();

        let expected = Interaction {
            id: "Interaction",
            pages: vec![
                Page {
                    metadata: PageMetadata::new_perm_double("Siva"),
                    content: "First page".to_owned(),
                },
                Page {
                    metadata: PageMetadata::new_perm_double("Terra"),
                    content: "Second page\nWith more words".to_owned(),
                },
            ],
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    #[ignore = "too complicated for now"]
    fn parse_one_interaction_many_pages() {
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

Testing

---

"#;

        let mut parser = DgParser::new();
        let parsed = parser.parse(data).unwrap();

        let expected = Interaction {
            id: "Interaction",
            pages: vec![
                Page {
                    metadata: PageMetadata::new_perm_double("Deez"),
                    content: "When the words are sus".to_owned(),
                },
                Page {
                    metadata: PageMetadata::new_perm_double("Gamer"),
                    content: "Words go brrr".to_owned(),
                },
                Page {
                    metadata: PageMetadata {
                        speaker: Metadata::NoChange,
                        vox: Metadata::NoChange,
                    },
                    content: "When the imposter is sus".to_owned(),
                },
                Page {
                    metadata: PageMetadata::new_perm_double("Siva"),
                    content: "Testing".to_owned(),
                },
            ],
        };

        assert_eq!(parsed, expected);
    }
}
