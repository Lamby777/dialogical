//!
//! Stuff for parsing dg files
//!

pub type Result<T> = std::result::Result<T, ParseError>;

use std::path::PathBuf;

use crate::comptime::{Script, ScriptPath};
use crate::consts::{COMPTIME_BORDER, SEPARATOR};
use crate::pages::{ChoicesState, Interaction, Page, ParseState};
use crate::InteractionMap;

mod context;
mod endings;
mod metaline;

pub use crate::pages::ParseError;
pub use context::ScriptContext;
pub use endings::{DialogueChoice, DialogueEnding, Label};

pub struct DgParser {
    state: ParseState,
    context: ScriptContext,

    /// Entry file path for resolving imports
    path: PathBuf,

    /// the end result it's putting together
    interactions: InteractionMap,

    // temp buffers for parsing
    // TODO store these inside `ParseState`
    interaction: Option<Interaction>,
    ix_id: Option<String>,
    comptime_script: Vec<String>,
    page: Page,
    pagebuf: Vec<String>,
    page_had_ending: bool,
}

impl DgParser {
    pub fn new(path: PathBuf) -> Self {
        Self {
            state: ParseState::default(),
            context: ScriptContext::default(),
            path,

            interactions: InteractionMap::new(),
            interaction: None,
            ix_id: None,
            page: Page::default(),
            pagebuf: vec![],
            comptime_script: vec![],
            page_had_ending: false,
        }
    }

    fn set_ix_id(&mut self, id: &str) -> Result<()> {
        if self.interaction.is_some() {
            self.push_ix()?;
        }

        self.ix_id = Some(id.to_owned());
        self.interaction = Some(Interaction::default());

        Ok(())
    }

    fn parse_comptime(&mut self, line: &str) -> Result<()> {
        // if current line is the closing `---`
        if line == SEPARATOR && self.comptime_script.last() == Some(&COMPTIME_BORDER.to_owned()) {
            self.comptime_script.pop();

            let content = self.comptime_script.join("\n");
            let path = ScriptPath(self.path.clone());
            let mut script = Script::new(content, path);
            script.execute(&mut self.context)?;

            // TODO no `self.script`, make the enum variant
            // store the script built up so far
            self.comptime_script.clear();

            self.state = match &self.state {
                ParseState::ComptimeScript(state) => *state.clone(),
                _ => unreachable!(),
            };
        } else {
            self.comptime_script.push(line.to_owned());
        }

        Ok(())
    }

    fn parse_message(&mut self, line: &str) -> Result<()> {
        // if parsing a message, add it to the result
        // OR stop parsing if empty line
        if line.is_empty() {
            self.state = ParseState::Choices(ChoicesState::Choices);
        } else {
            self.pagebuf.push(line.to_string());
        }

        Ok(())
    }

    /// push page buffer to the pages vec, then clear the buffer
    fn push_page(&mut self) -> Result<()> {
        // join each line with spaces unless they end in the
        // literal 2 characters "\n", in which case we replace
        // the \n with an actual newline
        self.page.content = {
            let mut it = self.pagebuf.iter().peekable();

            let mut res = String::new();
            while let Some(line) = it.next() {
                let to_push = if line.ends_with("\\n") {
                    line.replace("\\n", "\n")
                } else if it.peek().is_some() {
                    format!("{} ", line)
                } else {
                    line.clone()
                };

                res.push_str(&to_push);
            }

            res
        };

        let ix = self.interaction.as_mut().ok_or(ParseError::PushPageNoIX)?;

        // you may not add another page after an ending
        // for more info, see <https://github.com/Lamby777/dialogical/issues/3>
        let ix_has_ending_yet = ix.ending != DialogueEnding::End;
        if ix_has_ending_yet {
            if self.page_had_ending {
                return Err(ParseError::PageAfterEnding);
            }

            // "poisons" the current interaction so it remembers
            // that it had an ending until it's pushed
            self.page_had_ending = true;
        }

        ix.pages.push(self.page.clone());
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
        let ix_id = self.ix_id.take();
        let ix = self.interaction.take();
        let comptime_imports = self.context.drain_interactions();

        if let (Some(ix_id), Some(ix)) = (ix_id, ix) {
            if self.interactions.contains_key(&ix_id) {
                return Err(ParseError::PushDuplicateIX);
            }

            self.interactions.insert(ix_id, ix);
        } else if comptime_imports.is_empty() {
            // empty ix are not allowed... UNLESS there are
            // imports in a comptime script inside it
            return Err(ParseError::PushEmptyIX);
        }

        // push any interactions imported from comptime scripts
        self.interactions.extend(comptime_imports);

        self.page_had_ending = false;

        Ok(())
    }

    pub fn parse_all(&mut self, data: &str) -> Result<InteractionMap> {
        let lines = data.lines();

        self.pagebuf.clear();
        self.page = Page::default();

        for line in lines {
            use ParseState::*;

            let line = line.trim();

            match self.state {
                // besides the start, a block can either be
                // a comptime script or a message section
                ComptimeScript(_) => self.parse_comptime(line)?,

                Metadata => metaline::parse(self, line)?,
                Message => self.parse_message(line)?,
                Choices(ChoicesState::Choices) => endings::parse_choice(self, line)?,
            };
        }

        self.push_ix()?;
        let res = self.interactions.clone();
        self.interactions.clear();
        Ok(res)
    }
}

#[cfg(test)]
mod tests;
