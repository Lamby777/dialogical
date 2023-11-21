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

    #[error("Could not split {0}. Is this supposed to be metadata?")]
    NotMeta(String),

    #[error("{0} is not a valid metadata directive")]
    InvalidMeta(String),
}

type Result<T> = std::result::Result<T, ParseError>;

/// possible states the parser can be in
#[derive(Clone, Debug)]
enum ParseState {
    /// Interaction-wide metadata not set yet, we're at the
    /// top of an interaction
    Start,

    /// Waiting to start a new interaction or comptime script
    Idle,

    /// Compile-time script block
    ComptimeScript,

    /// Directives written before a message
    /// Separated from the actual message by an empty line
    Metadata,

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

    fn parse_idle(&mut self, line: &str, page: &mut Page) -> Result<()> {
        self.state = match line.trim() {
            "" => return Ok(()),
            "###" => ParseState::ComptimeScript,
            "%%%" => ParseState::Start,
            "---" => ParseState::Metadata,
            _ => panic!("wtf"),
        };

        Ok(())
    }

    fn parse_metaline(&mut self, line: &str, page: &mut Page) -> Result<()> {
        // TODO consider trimming whitespace before it gets
        // sent to any of these functions... might be a bad
        // idea to reduce the level of control these functions
        // have, but it would also reduce the complexity
        let line = line.trim();

        if line.is_empty() {
            self.state = ParseState::Message;
            return Ok(());
        }

        /////////////////////////////////////////////////////
        // TODO check if pageonly
        /////////////////////////////////////////////////////

        // everything after the space is the value
        //
        // WTF RUST THIS IS VALID SYNTAX?
        // BASED LANGUAGE
        let Some((key, val)) = line.split_once(' ') else {
            return Err(ParseError::NotMeta(line.to_string()));
        };

        match key {
            "NAME" => {
                let meta = Metadata::Permanent(val.to_owned());
                page.metadata.speaker = meta;
            }

            "VOX" => {
                let meta = Metadata::Permanent(val.to_owned());
                page.metadata.vox = meta;
            }
            _ => {
                return Err(ParseError::InvalidMeta(line.to_string()));
            }
        }

        Ok(())
    }

    fn parse_message(&mut self, line: &str, pagebuf: &mut Vec<String>) {
        // if parsing a message, add it to the result
        // OR stop parsing if empty line
        if line.is_empty() {
            self.state = ParseState::PostLine;
            return;
        }

        pagebuf.push(line.to_string());
    }

    // TODO: allow empty lines in message, and remove the last
    // empty line retroactively when it encounters a separator
    fn parse_postline(&mut self, line: &str, pagebuf: &mut Vec<String>, page: &Page) -> Result<()> {
        println!("Printing page... {}", line);

        if line != SEPARATOR {
            return Err(ParseError::AfterPostline(line.to_string()));
        }

        // push and reset the page buffer
        let mut page = page.clone();
        page.content = pagebuf.join("\n");
        self.pages.push(page);
        pagebuf.clear();

        println!("Printed");

        // TODO this is prob a bad idea for parsing
        // multiple interactions in one file
        self.state = ParseState::Metadata;
        Ok(())
    }

    pub fn parse(&mut self, data: &str) -> Result<Interaction> {
        println!("Parsing...");
        let lines = data.lines();

        // temporary buffer for the current page it's processing
        let mut pagebuf = vec![];

        // TODO maybe use MaybeUninit and partially initialize
        let mut page = Page {
            metadata: PageMetadata::default(),
            content: "".to_owned(),
        };

        for line in lines {
            use ParseState::*;

            println!("{:?} >> {:?}", &self.state, line);

            match self.state {
                Start => self.parse_start(line)?,
                Idle => self.parse_idle(line, &mut page)?,

                // besides the start, a block can either be
                // a comptime script or a message section
                ComptimeScript => todo!("comptime"),

                Metadata => self.parse_metaline(line, &mut page)?,
                Message => self.parse_message(line, &mut pagebuf),
                PostLine => self.parse_postline(line, &mut pagebuf, &page)?,
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
            id: "Test1",
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
