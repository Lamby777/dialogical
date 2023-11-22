//!
//! Compile-time scripting, basically macros but
//! will have some more features.
//!
//! (yes the name was inspired by Zig, but it's
//! nowhere near as powerful, just means "compile-time" :P)
//!

use std::{cell::RefCell, rc::Rc};
use thiserror::Error;

mod autolink;
use autolink::Autolink;

const COMMENT_PREFIX: &str = "//";

type Result<T> = std::result::Result<T, ScriptError>;

#[derive(Error, Debug, PartialEq)]
pub enum ScriptError {
    #[error("No such command")]
    NoSuchCommand,

    #[error("Incorrect usage of autolink")]
    InvalidAutolink,
}

#[derive(Default)]
enum ComptimeState {
    #[default]
    Normal,

    // the result is stored in the tuple
    Autolink(Autolink),
}

pub struct Script {
    content: String,
    state: Rc<RefCell<ComptimeState>>,
}

impl From<&str> for Script {
    fn from(content: &str) -> Self {
        Self {
            content: content.to_owned(),
            state: Rc::new(RefCell::new(ComptimeState::default())),
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

            "Autolink" => {
                let link_key = split.next().ok_or(ScriptError::InvalidAutolink)?;
                let link_target = split.collect::<Vec<_>>().join(" ");

                // TODO what happens if link_target is empty?

                let link = Autolink::new(link_key, &link_target);

                self.state.replace(ComptimeState::Autolink(link));
            }

            "Quit" => return Ok(true),

            _ => {
                return Err(ScriptError::NoSuchCommand);
            }
        };

        Ok(false)
    }

    fn execute_autolink(&self, line: &str, _out: &mut Vec<String>) -> Result<()> {
        if line.starts_with(COMMENT_PREFIX) {
            return Ok(());
        }

        Ok(())
    }

    pub fn execute(&mut self, out: &mut Vec<String>) -> Result<()> {
        let lines = self.content.lines();

        for line in lines {
            match *self.state.borrow() {
                ComptimeState::Normal => {
                    let should_quit = self.execute_normal(line, out)?;
                    if should_quit {
                        return Ok(());
                    }
                }

                ComptimeState::Autolink(_) => self.execute_autolink(line, out)?,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;
