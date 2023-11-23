//!
//! Stuff for parsing dg files
//!

type Result<T> = std::result::Result<T, ParseError>;

use crate::comptime::{Link, LinkKVPair, Script};
use crate::consts::{COMPTIME_BORDER, SEPARATOR};
use crate::pages::{Interaction, Metadata, Page, ParseError, ParseState, Speaker};

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
    links: Vec<Link>,
}

impl DgParser {
    fn set_ix_id(&mut self, id: &str) -> Result<()> {
        if self.interaction.is_some() {
            self.push_ix()?;
        }

        self.interaction = Some(Interaction {
            id: id.to_owned(),
            pages: vec![],
        });

        Ok(())
    }

    /// Takes a KV pair and returns all the links that
    /// target that specific pair.
    fn get_links_for(&mut self, kv: LinkKVPair) -> Vec<LinkKVPair> {
        self.links
            .iter()
            .filter_map(|v| {
                if v.from == kv {
                    Some(v.linked.clone())
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    fn parse_comptime(&mut self, line: &str) -> Result<()> {
        // if current line is the closing `---`
        if line == SEPARATOR && self.script.last() == Some(&COMPTIME_BORDER.to_owned()) {
            self.script.pop();

            let mut out = vec![];
            let script_content = self.script.join("\n");
            let mut script = Script::from(&script_content[..]);
            script.execute(&mut out, &mut self.links)?;

            self.state = match &self.state {
                ParseState::ComptimeScript(state) => *state.clone(),
                _ => unreachable!(),
            };
        } else {
            self.script.push(line.to_owned());
        }

        Ok(())
    }

    fn parse_metaline(&mut self, line: &str) -> Result<()> {
        // TODO consider trimming whitespace before it gets
        // sent to any of these functions... might be a bad
        // idea to reduce the level of control these functions
        // have, but it would also reduce the complexity
        let line = line.trim();

        // TODO break up this function (wtf like 4 billion lines)

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

        /// split into first "word" and the rest
        fn split_first_whitespace(full: &str) -> Result<(&str, &str)> {
            full.split_once(char::is_whitespace)
                .ok_or(ParseError::NotMeta(full.to_string()))
                .map(|(k, v)| (k, v.trim_start()))
        }

        let (kv, pageonly) = {
            // everything after the space is the value
            let kv = split_first_whitespace(line)?;

            // ...unless the key is PageOnly, in which case we
            // repeat the process again
            if kv.0 == "PageOnly" {
                (split_first_whitespace(kv.1)?, true)
            } else {
                (kv, false)
            }
        };

        match kv.0 {
            "SOMEONE" => {
                self.page.metadata.speaker = Metadata::new(Speaker::Unknown, pageonly);
                return Ok(());
            }

            // message spoken by narrator...
            // how this will be interpreted is an implementation detail
            "NARRATOR" => {
                self.page.metadata.speaker = Metadata::new(Speaker::Narrator, pageonly);
                return Ok(());
            }

            "%" if !pageonly => {
                return self.set_ix_id(kv.1);
            }

            _ => {}
        }

        // the pair + any pairs linked using the `Link` directive
        let pair = LinkKVPair::from_tuple(kv);
        let links = self.get_links_for(pair);
        let mapped = links.iter().map(|v| (v.0.as_str(), v.1.as_str()));
        let kvpairs = std::iter::once(kv).chain(mapped);

        for (key, val) in kvpairs {
            match key {
                "NAME" => {
                    let name = Speaker::Named(val.to_owned());
                    self.page.metadata.speaker = Metadata::new(name, pageonly)
                }

                "VOX" => self.page.metadata.vox = Metadata::new(val.to_owned(), pageonly),

                _ => {
                    return Err(ParseError::InvalidMeta(line.to_string()));
                }
            };
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

    // TODO allow empty lines in message, and remove the last
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
    fn push_page(&mut self) {
        self.page.content = self.pagebuf.join("\n");

        self.interaction
            .as_mut()
            .unwrap()
            .pages
            .push(self.page.clone());

        self.pagebuf.clear();
        self.page = Page::default();
    }

    /// Finish parsing 1 interaction, and clear the state
    /// to prepare for another one.
    ///
    /// `Err` if the parser is in a state where it's not
    /// prepared to finish just yet.
    fn push_ix(&mut self) -> Result<()> {
        self.interactions
            .push(self.interaction.take().ok_or(ParseError::PushEmpty)?);
        Ok(())
    }

    pub fn parse_all(&mut self, data: &str) -> Result<&[Interaction]> {
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

        println!("{:?}", self.links);

        self.push_ix()?;
        Ok(&self.interactions)
    }
}

#[cfg(test)]
mod tests;
