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

        fn script_path(split: SplitWhitespace) -> Result<ScriptPath> {
            let args = split.collect::<Vec<_>>().join(" ");
            let path = PathBuf::from(args);
            Ok(ScriptPath(path))
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
                let interactions = script_path(split)?.parse()?;
            }

            "Execute" => {
                let content = script_path(split)?.resolve()?;
                Script::from(content).execute(out, links)?;
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

        if !line.is_empty() {
            // add another property to the list of
            // things the current link will point to
            let mut split = line.split_whitespace();
            let pair = LinkKVPair::from_words(&mut split)?;
            link.add_link(pair);
            return Ok(None);
        }

        // we're done building the link, so...
        if !link.negative {
            // push it to the parser's list of links
            links.push(link.clone());
        } else {
            // OR if negative, go through all links that have
            // the same `from` and remove the `linked` properties
            // they have in common with the negative link
            let _ = links
                .iter_mut()
                .filter(|other| other.from == link.from)
                .map(|v| {
                    v.linked.retain(|other| !link.linked.contains(other));
                });
        }

        Ok(Some(ComptimeState::Normal))
    }

    pub fn execute(&mut self, out: &mut Vec<String>, links: &mut Vec<Link>) -> Result<()> {
        use ComptimeState::*;
        let lines = self.content.lines().chain(std::iter::once(""));

        // take one line at a time...
        // remembers which mode we're in

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
