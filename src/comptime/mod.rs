//!
//! Compile-time scripting, basically macros but
//! will have some more features.
//!
//! (yes the name was inspired by Zig, but it's
//! nowhere near as powerful, just means "compile-time" :P)
//!
use std::{cell::RefCell, rc::Rc};

use thiserror::Error;

const COMMENT_PREFIX: &str = "//";

#[derive(Error, Debug, PartialEq)]
pub enum ScriptError {
    #[error("Panic for whatever dumb reason")]
    TestPanic,
}

type Result<T> = std::result::Result<T, ScriptError>;

#[derive(Default)]
enum ComptimeState {
    #[default]
    Normal,

    Autolink,
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

struct Autolinker {
    key: String,
    value: String,
    linked: Vec<String>,
}

impl Script {
    fn execute_normal(&self, line: &str, out: &mut Vec<String>) -> Result<()> {
        if line.starts_with(COMMENT_PREFIX) {
            return Ok(());
        }

        // split into key and value, but the value is optional
        let mut split = line.split_whitespace();
        let Some(key) = split.next() else {
            return Ok(());
        };

        match key {
            "Echo" => {
                let rest = split.collect::<Vec<_>>().join(" ");
                out.push(rest.to_owned());
            }

            "Autolink" => {
                self.state.replace(ComptimeState::Autolink);
            }

            "Quit" => return Ok(()),

            _ => {
                return Err(ScriptError::TestPanic);
            }
        };

        Ok(())
    }

    pub fn execute(&mut self, out: &mut Vec<String>) -> Result<()> {
        let lines = self.content.lines();

        for line in lines {
            match *self.state.borrow() {
                ComptimeState::Normal => self.execute_normal(line, out)?,
                ComptimeState::Autolink => todo!(),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! comptime {
        ($code:expr) => {{
            let mut out = vec![];
            let res = Script::from($code).execute(&mut out);
            (res, out)
        }};
    }

    #[test]
    fn quit_cmd() {
        let (res, out) = comptime!(
            r#"
        Echo I just wanted to say...
        Quit
        Echo I love you.
        "#
        );

        assert!(res.is_ok());
        assert_eq!(out, vec!["I just wanted to say...".to_owned()]);
    }

    #[test]
    fn blanked_out() {
        let (res, out) = comptime!(
            r#"


        "#
        );
        assert!(res.is_ok());
        assert_eq!(out.len(), 0);
    }

    #[test]
    fn hello_world() {
        let (res, out) = comptime!(r#"Echo Hello, world!"#);
        assert!(res.is_ok());
        assert_eq!(out, vec!["Hello, world!".to_string()]);
    }
}
