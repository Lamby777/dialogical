//!
//! Compile-time scripting, basically macros but
//! will have some more features.
//!
//! (yes the name was inspired by Zig, but it's
//! nowhere near as powerful, just means "compile-time" :P)
//!

use std::cell::RefCell;
use std::path::PathBuf;
use std::str::SplitWhitespace;
use thiserror::Error;

use crate::consts::PREFIX_COMMENT;
use crate::pages::ParseError;
use crate::parser::ScriptContext;
use crate::Interaction;

mod include;
mod link;

pub use include::ScriptPath;
pub use link::{Link, LinkKVPair, Unlink};

pub type Result<T> = std::result::Result<T, ScriptError>;

#[derive(Error, Debug, PartialEq)]
pub enum ScriptError {
    #[error("No such command")]
    NoSuchCommand,

    #[error("Incorrect usage of Link directive")]
    InvalidLink,

    #[error("Attempt to link 2 of the same property in associations")]
    DoubleLink,

    #[error("Attempt to link a target to be associated with itself")]
    TargetToSelf,

    #[error("Could not open file at path {0}")]
    FileOpen(PathBuf),

    #[error("Error while importing interactions from script at path {0}")]
    Import(PathBuf, #[source] Box<ParseError>),
}

#[derive(Clone, Debug, Default)]
enum ComptimeState {
    #[default]
    Normal,

    /// Exit at end of interpreter loop
    Quit,

    /// the result is stored in the tuple
    Link(Link),
}

pub struct Script {
    content: String,
    state: RefCell<ComptimeState>,
    path: ScriptPath,
}

// stuff passed back to the parser once the script is done
#[derive(Clone, Debug, PartialEq)]
pub enum ScriptOutput {
    LogMessage(String),
    Link(Link),
    Interaction(Interaction),
}

impl Script {
    pub fn new(content: String, path: ScriptPath) -> Self {
        Self {
            content,
            state: RefCell::new(ComptimeState::default()),
            path,
        }
    }

    /// returns the new state (`None` = no change)
    fn execute_normal(&self, line: &str, out: &mut ScriptContext) -> Result<Option<ComptimeState>> {
        if line.starts_with(PREFIX_COMMENT) {
            return Ok(None);
        }

        // split into command and args, but the args are optional
        let mut split = line.split_whitespace();
        let Some(command) = split.next() else {
            return Ok(None);
        };

        // join the iterator into a script path
        fn script_path(script: &Script, split: SplitWhitespace) -> ScriptPath {
            let args = split.collect::<Vec<_>>().join(" ");
            let path = PathBuf::from(args);

            script.path.make_append(path)
        }

        match command {
            "Echo" => {
                out.log(&split.collect::<Vec<_>>().join(" "));
            }

            "Link" => {
                // TODO what happens the target part is empty?
                let pair = LinkKVPair::from_words(&mut split)?;
                let link = Link::from_pair(pair);

                return Ok(Some(ComptimeState::Link(link)));
            }

            "Unlink" => {
                let pair = LinkKVPair::from_words(&mut split)?;
                let link = Link::from_pair(pair);
                // link.negative = command == "Unlink";

                return Ok(Some(ComptimeState::Link(link)));
            }

            "Import" => {
                // Isolates the returned links from any other
                // side effects the script might have.
                // Might need to do more than just this
                // later on when the language has more features.
                let path = script_path(self, split);
                let interactions = path
                    .parse()
                    .map_err(|e| ScriptError::Import(path.0, Box::new(e)))?;

                let mapped = interactions
                    .into_iter()
                    .map(|v| ScriptOutput::Interaction(v));

                out.0.extend(mapped);
            }

            "Execute" => {
                // TODO this probably isn't doing what it should
                let path = script_path(self, split);
                let content = path.read()?;
                let mut script = Self::new(content, path);
                script.execute(out)?;
            }

            "Quit" => return Ok(Some(ComptimeState::Quit)),

            _ => {
                return Err(ScriptError::NoSuchCommand);
            }
        };

        Ok(None)
    }

    fn execute_unlink(
        &self,
        line: &str,
        out: &mut ScriptContext,
        unlink: &mut Unlink,
    ) -> Result<Option<ComptimeState>> {
        let line = line.trim();
        if line.starts_with(PREFIX_COMMENT) {
            return Ok(None);
        }

        if !line.is_empty() {
            // add another property to the list of
            // things the current link will point to
            unlink.add_association(line.to_owned());
            return Ok(None);
        }

        Ok(Some(ComptimeState::Normal))
    }

    fn execute_link(
        &self,
        line: &str,
        out: &mut ScriptContext,
        link: &mut Link,
    ) -> Result<Option<ComptimeState>> {
        if line.starts_with(PREFIX_COMMENT) {
            return Ok(None);
        }

        if !line.is_empty() {
            // add another property to the list of
            // things the current link will point to
            let mut split = line.split_whitespace();
            let pair = LinkKVPair::from_words(&mut split)?;
            link.add_association(pair);
            return Ok(None);
        }

        // Some checks to make sure you aren't being a dumbass
        // <https://github.com/Lamby777/dialogical/issues/2>
        let is_dupe = out.iter_links().any(|v| v.target == link.target);
        if is_dupe {
            return Err(ScriptError::DoubleLink);
        }

        // we're done building the link, so...
        // if !link.negative { TODO
        if true {
            // push it to the parser's list of links
            out.link(link.clone());
        } else {
            // OR if negative, go through all links that have
            // the same `from` and remove the `linked` properties
            // they have in common with the negative link
            out.unlink(&link);
        }

        Ok(Some(ComptimeState::Normal))
    }

    pub fn execute(&mut self, out: &mut ScriptContext) -> Result<()> {
        use ComptimeState::*;
        let lines = self.content.lines().chain(std::iter::once(""));

        // take one line at a time...
        // remembers which mode we're in

        for line in lines {
            let new_state = match *self.state.borrow_mut() {
                Normal => self.execute_normal(line, out)?,
                Link(ref mut link) => self.execute_link(line, out, link)?,

                Quit => unreachable!(),
            };

            match new_state {
                Some(Quit) => {
                    return Ok(());
                }

                Some(state) => {
                    self.state.replace(state);
                }

                _ => (),
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;
