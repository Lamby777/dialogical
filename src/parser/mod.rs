//!
//! Stuff for parsing dg files
//!

pub type Result<T> = std::result::Result<T, ParseError>;

use std::path::PathBuf;

use crate::comptime::{Script, ScriptPath};
use crate::consts::{COMPTIME_BORDER, SEPARATOR};
use crate::pages::{Interaction, Page, ParseError, ParseState};

mod context;
mod dendings;
mod metaline;

pub use context::ScriptContext;
pub use dendings::{DialogueChoice, DialogueEnding, Label};

pub struct DgParser {
    state: ParseState,
    context: ScriptContext,

    /// Entry file path for resolving imports
    path: PathBuf,

    /// the end result it's putting together
    interactions: Vec<Interaction>,

    // temp buffers for parsing
    // TODO store these inside `ParseState`
    interaction: Option<Interaction>,
    script: Vec<String>,
    page: Page,
    pagebuf: Vec<String>,
    choices: Option<DialogueChoice>,
}

impl DgParser {
    pub fn new(path: PathBuf) -> Self {
        Self {
            state: ParseState::default(),
            context: ScriptContext::default(),
            path,

            interactions: vec![],
            interaction: None,
            page: Page::default(),
            pagebuf: vec![],
            choices: None,
            script: vec![],
        }
    }

    fn set_ix_id(&mut self, id: &str) -> Result<()> {
        if self.interaction.is_some() {
            self.push_ix()?;
        }

        self.interaction = Some(Interaction::new_with_id(id));

        Ok(())
    }

    fn parse_comptime(&mut self, line: &str) -> Result<()> {
        // if current line is the closing `---`
        if line == SEPARATOR && self.script.last() == Some(&COMPTIME_BORDER.to_owned()) {
            self.script.pop();

            let content = self.script.join("\n");
            let path = ScriptPath(self.path.clone());
            let mut script = Script::new(content, path);
            script.execute(&mut self.context)?;

            // TODO no `self.script`, make the enum variant
            // store the script built up so far
            self.script.clear();

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
            self.state = ParseState::Choices;
        } else {
            self.pagebuf.push(line.to_string());
        }

        Ok(())
    }

    fn parse_choices(&mut self, line: &str) -> Result<()> {
        match self.choices.take() {
            // if the line is an option, parse it
            Some(ending) => {
                let inter = self.interaction.as_mut().unwrap();

                Ok(())
            }

            // if the line is a separator and we're not in the
            // middle of parsing an ending, then we're done.
            //
            // push the page and move on.
            None if line == SEPARATOR => {
                self.push_page()?;
                self.state = ParseState::Metadata;
                Ok(())
            }

            _ => {
                // TODO ???
                todo!()
            }
        }
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
        self.choices.take(); // TODO is this line necessary?
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

        // push any interactions imported from comptime scripts
        self.interactions.extend(self.context.drain_interactions());

        Ok(())
    }

    pub fn parse_all(&mut self, data: &str) -> Result<&[Interaction]> {
        let lines = data.lines();

        self.pagebuf.clear();
        self.page = Page::default();

        for line in lines {
            use ParseState::*;

            match self.state {
                // besides the start, a block can either be
                // a comptime script or a message section
                ComptimeScript(_) => self.parse_comptime(line)?,

                Metadata => self.parse_metaline(line)?,
                Message => self.parse_message(line)?,
                Choices => self.parse_choices(line)?,
            };
        }

        self.push_ix()?;
        Ok(&self.interactions)
    }
}

#[cfg(test)]
mod tests;
