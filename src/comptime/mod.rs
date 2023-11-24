//!
//! Compile-time scripting, basically macros but
//! will have some more features.
//!
//! (yes the name was inspired by Zig, but it's
//! nowhere near as powerful, just means "compile-time" :P)
//!

use std::cell::RefCell;
use thiserror::Error;

use crate::consts::COMMENT_PREFIX;

mod include;
mod link;

pub use link::{Link, LinkKVPair};

pub type Result<T> = std::result::Result<T, ScriptError>;

#[derive(Error, Debug, PartialEq)]
pub enum ScriptError {
    #[error("No such command")]
    NoSuchCommand,

    #[error("Incorrect usage of Link directive")]
    InvalidLink,
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

impl From<&str> for Script {
    fn from(content: &str) -> Self {
        Self {
            content: content.to_owned(),
            state: RefCell::new(ComptimeState::default()),
        }
    }
}

impl Script {
    /// result is the new state (`None` = no change)
    fn execute_normal(&self, line: &str, out: &mut Vec<String>) -> Result<Option<ComptimeState>> {
        if line.starts_with(COMMENT_PREFIX) {
            return Ok(None);
        }

        // split into key and value, but the value is optional
        let mut split = line.split_whitespace();
        let Some(key) = split.next() else {
            return Ok(None);
        };

        match key {
            "Echo" => {
                let rest = split.collect::<Vec<_>>().join(" ");
                out.push(rest.to_owned());
            }

            "Link" => {
                // TODO what happens the target part is empty?
                let pair = LinkKVPair::from_words(&mut split)?;
                let link = Link::from_pair(pair);

                return Ok(Some(ComptimeState::Link(link)));
            }

            "Unlink" => {
                // TODO try to use ComptimeState::Link, and not a separate variant
                todo!()
            }

            "Include" => {
                todo!()
            }

            "Import" => {
                todo!()
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
                Normal => self.execute_normal(line, out)?,
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
