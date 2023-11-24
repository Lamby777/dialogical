//!
//! Stuff for parsing dg files
//!

pub type Result<T> = std::result::Result<T, ParseError>;

use crate::comptime::{Link, LinkKVPair, Script};
use crate::consts::{COMPTIME_BORDER, SEPARATOR};
use crate::pages::{Interaction, Page, ParseError, ParseState};

mod metaline;

#[derive(Default)]
pub struct DgParser {
    state: ParseState,

    // the end result it's putting together
    interactions: Vec<Interaction>,

    // temp buffers for parsing
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
            let mut script = Script::from(script_content);
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
        let line = line.trim();

        // empty line = end of metadata
        if line.is_empty() {
            self.state = ParseState::Message;
            return Ok(());
        }

        metaline::parse(self, line)
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

        self.push_page()?;

        self.state = ParseState::Metadata;
        Ok(())
    }

    /// push page buffer to the pages vec, then clear the buffer
    fn push_page(&mut self) -> Result<()> {
        self.page.content = self.pagebuf.join("\n");

        self.interaction
            .as_mut()
            .ok_or(ParseError::PushPageNoIX)?
            .pages
            .push(self.page.clone());

        self.pagebuf.clear();
        self.page = Page::default();

        Ok(())
    }

    /// Finish parsing 1 interaction, and clear the state
    /// to prepare for another one.
    ///
    /// `Err` if the parser is in a state where it's not
    /// prepared to finish just yet.
    fn push_ix(&mut self) -> Result<()> {
        self.interactions
            .push(self.interaction.take().ok_or(ParseError::PushEmptyIX)?);
        Ok(())
    }

    pub fn parse_all(&mut self, data: &str) -> Result<&[Interaction]> {
        let lines = data.lines();

        self.pagebuf.clear();
        self.page = Page::default();

        for line in lines {
            use ParseState::*;

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
