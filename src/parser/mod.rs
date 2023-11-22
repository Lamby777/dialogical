//!
//! Stuff for parsing dg files
//!

const COMPTIME_BORDER: &str = "###";
const SEPARATOR: &str = "---";
type Result<T> = std::result::Result<T, ParseError>;

// TODO don't wildcard import
use crate::{comptime::Script, pages::*};

#[derive(Default)]
pub struct DgParser {
    state: ParseState,

    // the end result it's putting together
    interactions: Vec<Interaction>,

    // temp buffers for parsing
    // TODO maybe use MaybeUninit and partially initialize
    interaction: Option<Interaction>,
    page: Page,
    pagebuf: Vec<String>,

    script: Vec<String>,
}

impl DgParser {
    fn set_ix_id(&mut self, id: &str) -> Result<()> {
        if let Some(ix) = &self.interaction {
            println!("End of current interaction {}", ix.id);
            self.push_ix()?;
        } else {
            println!("Not some");
        }

        self.interaction = Some(Interaction {
            id: id.to_owned(),
            pages: vec![],
        });

        Ok(())
    }

    fn parse_comptime(&mut self, line: &str) -> Result<()> {
        // TODO don't send entire line to script
        if line != COMPTIME_BORDER {
            self.script.push(line.to_owned());
            return Ok(());
        }

        let mut out = vec![];
        let script_content = self.script.join("\n");
        let script = Script::from(&script_content[..]);
        script.execute(&mut out)?;

        self.state = match &self.state {
            ParseState::ComptimeScript(state) => *state.clone(),
            _ => unreachable!(),
        };

        return Ok(());
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

        // enter comptime scripting block
        if line == COMPTIME_BORDER {
            // comptime script inside a comptime script is 100% a parsing error
            debug_assert!(!matches!(self.state, ParseState::ComptimeScript(_)));

            self.state = ParseState::ComptimeScript(Box::new(self.state.clone()));
            return Ok(());
        }

        let (key, val, pageonly) = {
            // everything after the space is the value
            let (mut key, mut val) = line
                .split_once(' ')
                .ok_or(ParseError::NotMeta(line.to_string()))?;

            // ...unless the key is PageOnly, in which case we
            // repeat the process again
            let pageonly = key == "PageOnly";
            if pageonly {
                (key, val) = val
                    .split_once(' ')
                    .ok_or(ParseError::NotMeta(line.to_string()))?;
            }

            (key.trim(), val.trim(), pageonly)
        };

        match key {
            "NAME" => {
                let name = Speaker::Name(val.to_owned());
                self.page.metadata.speaker = Metadata::new(name, pageonly)
            }

            "VOX" => self.page.metadata.vox = Metadata::new(val.to_owned(), pageonly),

            // message spoken by narrator...
            // how this will be interpreted is an implementation detail
            "NARRATOR" => {
                self.page.metadata.speaker = Metadata::new(Speaker::Narrator, pageonly);
            }

            // set interaction
            "%" => self.set_ix_id(val)?,

            _ => {
                return Err(ParseError::InvalidMeta(line.to_string()));
            }
        };

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

        self.state = ParseState::Metadata;
        Ok(())
    }

    /// push page buffer to the pages vec, then clear the buffer
    pub fn push_page(&mut self) {
        self.page.content = self.pagebuf.join("\n");

        self.interaction
            .as_mut()
            .unwrap()
            .pages
            .push(self.page.clone());

        self.pagebuf.clear();
        self.page = Page::default();
        println!("Printed!");
    }

    /// Finish parsing 1 interaction, and clear the state
    /// to prepare for another one.
    ///
    /// `Err` if the parser is in a state where it's not
    /// prepared to finish just yet.
    pub fn push_ix(&mut self) -> Result<()> {
        self.interactions
            .push(self.interaction.take().ok_or(ParseError::PushEmpty)?);
        Ok(())
    }

    pub fn parse_all(&mut self, data: &str) -> Result<&[Interaction]> {
        println!("Parsing...");
        let lines = data.lines();

        self.pagebuf.clear();
        self.page = Page::default();

        for line in lines {
            use ParseState::*;

            println!("{:?} >> {:?}", &self.state, line);

            (match self.state {
                // besides the start, a block can either be
                // a comptime script or a message section
                ComptimeScript(_) => Self::parse_comptime,

                Metadata => Self::parse_metaline,
                Message => Self::parse_message,
                PostLine => Self::parse_postline,
            })(self, line)?;
        }

        self.push_ix()?;
        Ok(&self.interactions)
    }
}

#[cfg(test)]
mod tests;
