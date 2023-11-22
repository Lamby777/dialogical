//!
//! Compile-time scripting, basically macros but
//! will have some more features.
//!
//! (yes the name was inspired by Zig, but it's
//! nowhere near as powerful, just means "compile-time" :P)
//!
use thiserror::Error;

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
    state: ComptimeState,
}

impl From<&str> for Script {
    fn from(content: &str) -> Self {
        Self {
            content: content.to_owned(),
            state: ComptimeState::default(),
        }
    }
}

impl Script {
    pub fn execute(&self, out: &mut Vec<String>) -> Result<()> {
        let lines = self.content.lines();

        for line in lines {
            // split into key and value, but the value is optional
            let mut split = line.split_whitespace();
            let Some(key) = split.next() else {
                continue;
            };

            dbg!(&key);
            println!("{}", key.is_empty());

            match key {
                "Echo" => {
                    let rest = split.collect::<Vec<_>>().join(" ");
                    out.push(rest.to_owned());
                }

                "Quit" => return Ok(()),

                _ => {
                    return Err(ScriptError::TestPanic);
                }
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
