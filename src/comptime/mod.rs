//!
//! Compile-time scripting, basically macros but
//! will have some more features.
//!
//! (yes the name was inspired by Zig, but it's
//! nowhere near as powerful, just means "compile-time" :P)
//!
use thiserror::Error;

#[derive(Error, Debug)]
enum ScriptError {
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

struct Script {
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
    fn execute(&self, out: &mut Vec<String>) -> Result<()> {
        let lines = self.content.lines();

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }

            // split into key and value, but the value is optional
            let (key, val) = line.split_once(' ').unwrap_or((line, ""));

            match key {
                "Echo" => {
                    out.push(val.to_owned());
                }

                "###" => return Ok(()),

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
    fn block_end() {
        let (res, out) = comptime!(
            r#"
        Echo I just wanted to say...
        ###
        I love you.
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
