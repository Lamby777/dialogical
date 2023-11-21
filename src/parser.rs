//!
//! Stuff for parsing dg files
//!

const SEPARATOR: &str = "---";
type Result<T> = std::result::Result<T, ParseError>;

// TODO don't wildcard import
use crate::pages::*;

#[derive(Default)]
pub struct DgParser {
    state: ParseState,
    pages: Vec<Page>,
    interaction_id: Option<String>,

    // temp buffers for parsing
    // TODO maybe use MaybeUninit and partially initialize
    page: Page,
    pagebuf: Vec<String>,
}

impl DgParser {
    /// Finish parsing 1 interaction, and clear the state
    /// to prepare for another one.
    ///
    /// `Err` if the parser is in a state where it's not
    /// prepared to finish just yet.
    fn build_result(&self) -> Result<Interaction> {
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

    fn parse_idle(&mut self, line: &str) -> Result<()> {
        self.state = match line.trim() {
            "" => return Ok(()),
            "###" => ParseState::ComptimeScript,
            "%%%" => ParseState::Start,
            "---" => ParseState::Metadata,
            _ => panic!("wtf"),
        };

        Ok(())
    }

    fn parse_metaline(&mut self, line: &str) -> Result<()> {
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
                self.page.metadata.speaker = meta;
            }

            "VOX" => {
                let meta = Metadata::Permanent(val.to_owned());
                self.page.metadata.vox = meta;
            }
            _ => {
                return Err(ParseError::InvalidMeta(line.to_string()));
            }
        }

        Ok(())
    }

    fn parse_message(&mut self, line: &str) -> Result<()> {
        // if parsing a message, add it to the result
        // OR stop parsing if empty line
        if line.is_empty() {
            self.state = ParseState::PostLine;
        } else {
            self.pagebuf.push(line.to_string());
        }

        Ok(())
    }

    // TODO: allow empty lines in message, and remove the last
    // empty line retroactively when it encounters a separator
    fn parse_postline(&mut self, line: &str) -> Result<()> {
        if line != SEPARATOR {
            return Err(ParseError::AfterPostline(line.to_string()));
        }

        self.push_page();

        // TODO this is prob a bad idea for parsing
        // multiple interactions in one file
        self.state = ParseState::Metadata;
        Ok(())
    }

    /// push page buffer to the pages vec, then clear the buffer
    pub fn push_page(&mut self) {
        self.page.content = self.pagebuf.join("\n");
        self.pages.push(self.page.clone());
        self.pagebuf.clear();
        println!("Printed!");
    }

    pub fn parse(&mut self, data: &str) -> Result<Interaction> {
        println!("Parsing...");
        let lines = data.lines();

        self.pagebuf.clear();
        self.page = Page::default();

        for line in lines {
            use ParseState::*;

            println!("{:?} >> {:?}", &self.state, line);

            (match self.state {
                Start => Self::parse_start,
                Idle => Self::parse_idle,

                // besides the start, a block can either be
                // a comptime script or a message section
                ComptimeScript => todo!("comptime"),

                Metadata => Self::parse_metaline,
                Message => Self::parse_message,
                PostLine => Self::parse_postline,
            })(self, line)?;
        }

        self.build_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_small_interaction() {
        // you're giving me some real small ix energy right now
        let data = include_str!("../dummy-data/small-ix.dg");

        let mut parser = DgParser::default();
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
        let data = include_str!("../dummy-data/one-ix-many-pages.dg");

        let mut parser = DgParser::default();
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
