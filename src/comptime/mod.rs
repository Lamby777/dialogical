//!
//! Compile-time scripting, basically macros but
//! will have some more features.
//!
//! (yes the name was inspired by Zig, but it's
//! nowhere near as powerful, just means "compile-time" :P)
//!

use std::cell::RefCell;
use thiserror::Error;

use crate::comptime::link::LinkKVPair;
use crate::consts::COMMENT_PREFIX;

mod link;
pub use link::Link;

type Result<T> = std::result::Result<T, ScriptError>;

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

    // the result is stored in the tuple
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
    /// result is whether or not to quit early
    fn execute_normal(&self, line: &str, out: &mut Vec<String>) -> Result<bool> {
        if line.starts_with(COMMENT_PREFIX) {
            return Ok(false);
        }

        // split into key and value, but the value is optional
        let mut split = line.split_whitespace();
        let Some(key) = split.next() else {
            return Ok(false);
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

                self.state.replace(ComptimeState::Link(link));
            }

            "Quit" => return Ok(true),

            _ => {
                return Err(ScriptError::NoSuchCommand);
            }
        };

        Ok(false)
    }

    fn execute_link(
        &self,
        line: &str,
        _out: &mut Vec<String>,
        links: &mut Vec<Link>,
    ) -> Result<()> {
        if line.starts_with(COMMENT_PREFIX) {
            return Ok(());
        }

        if line.is_empty() {
            let link = self.state.replace(ComptimeState::Normal);
            if let ComptimeState::Link(link) = link {
                links.push(link);
            } else {
                unreachable!()
            }

            return Ok(());
        }

        let mut split = line.split_whitespace();
        let pair = LinkKVPair::from_words(&mut split)?;

        // Add the link
        //
        // NOTE: THIS is how you mutate the value inside
        // an enum variant wrapped in a RefCell. WTF?
        if let ComptimeState::Link(link) = &mut *self.state.borrow_mut() {
            link.add_link(pair);
        } else {
            // maybe remove this branch later after testing
            unreachable!();
        }

        Ok(())
    }

    pub fn execute(&mut self, out: &mut Vec<String>, links: &mut Vec<Link>) -> Result<()> {
        let lines = self.content.lines();

        for line in lines {
            let state = self.state.clone();
            let state = state.borrow();

            println!("state: {:?}", state);

            match *state {
                ComptimeState::Normal => {
                    let should_quit = self.execute_normal(line, out)?;
                    if should_quit {
                        return Ok(());
                    }
                }

                ComptimeState::Link(_) => self.execute_link(line, out, links)?,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;
