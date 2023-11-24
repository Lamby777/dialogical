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

use crate::consts::COMMENT_PREFIX;

mod include;
mod link;

pub use include::ScriptPath;
pub use link::{Link, LinkKVPair};

pub type Result<T> = std::result::Result<T, ScriptError>;

#[derive(Error, Debug, PartialEq)]
pub enum ScriptError {
    #[error("No such command")]
    NoSuchCommand,

    #[error("Incorrect usage of Link directive")]
    InvalidLink,

    #[error("Could not open file at path {0}")]
    FileOpenError(PathBuf),
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
}

impl From<String> for Script {
    fn from(content: String) -> Self {
        Self {
            content,
            state: RefCell::new(ComptimeState::default()),
        }
    }
}

impl From<&str> for Script {
    fn from(content: &str) -> Self {
        Self {
            content: content.to_owned(),
            state: RefCell::new(ComptimeState::default()),
        }
    }
}

impl Script {
    /// returns the new state (`None` = no change)
    fn execute_normal(
        &self,
        line: &str,
        out: &mut Vec<String>,
        links: &mut Vec<Link>,
    ) -> Result<Option<ComptimeState>> {
        if line.starts_with(COMMENT_PREFIX) {
            return Ok(None);
        }

        // split into command and args, but the args are optional
        let mut split = line.split_whitespace();
        let Some(command) = split.next() else {
            return Ok(None);
        };

        fn resolve_script(split: SplitWhitespace) -> Result<Script> {
            let args = split.collect::<Vec<_>>().join(" ");
            let path = PathBuf::from(args);
            let contents = ScriptPath(path).resolve()?;
            Ok(Script::from(contents))
        }

        match command {
            "Echo" => {
                out.push(split.collect::<Vec<_>>().join(" "));
            }

            "Link" | "Unlink" => {
                // TODO what happens the target part is empty?
                let pair = LinkKVPair::from_words(&mut split)?;
                let mut link = Link::from_pair(pair);
                link.negative = command == "Unlink";

                return Ok(Some(ComptimeState::Link(link)));
            }

            "Import" => {
                // Isolates the returned links from any other
                // side effects the script might have.
                // Might need to do more than just this
                // later on when the language has more features.
                resolve_script(split)?.execute(&mut vec![], links)?;
            }

            "Execute" => {
                resolve_script(split)?.execute(out, links)?;
            }

            "Quit" => return Ok(Some(ComptimeState::Quit)),

            _ => {
                return Err(ScriptError::NoSuchCommand);
            }
        };

        Ok(None)
    }

    fn execute_link(
        &self,
        line: &str,
        _out: &mut Vec<String>,
        link: &mut Link,
        links: &mut Vec<Link>,
    ) -> Result<Option<ComptimeState>> {
        if line.starts_with(COMMENT_PREFIX) {
            return Ok(None);
        }

        if line.is_empty() {
            links.push(link.clone());
            return Ok(Some(ComptimeState::Normal));
        }

        let mut split = line.split_whitespace();
        let pair = LinkKVPair::from_words(&mut split)?;
        link.add_link(pair);

        Ok(None)
    }

    pub fn execute(&mut self, out: &mut Vec<String>, links: &mut Vec<Link>) -> Result<()> {
        use ComptimeState::*;
        let lines = self.content.lines().chain(std::iter::once(""));

        for line in lines {
            let new_state = match *self.state.borrow_mut() {
                Normal => self.execute_normal(line, out, links)?,
                Link(ref mut link) => self.execute_link(line, out, link, links)?,

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
