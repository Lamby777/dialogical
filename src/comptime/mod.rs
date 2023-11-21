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
    fn exec_comptime(&self, out: &mut Vec<String>) -> Result<()> {
        let lines = self.content.lines();

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }

            let (key, val) = line.split_once(' ').ok_or(ScriptError::TestPanic)?;

            match key {
                "Echo" => {
                    out.push(val.to_owned());
                }
                _ => {
                    return Err(ScriptError::TestPanic);
                }
            }
        }

        Ok(())
    }
}

macro_rules! comptime {
    ($code:expr) => {{
        let mut out = vec![];
        let res = Script::from($code).exec_comptime(&mut out);
        (res, out)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blanked_out() {
        let script = Script::from(
            r#"


        "#,
        );
        let mut out = vec![];
        let res = script.exec_comptime(&mut out);
        assert!(res.is_ok());
        assert_eq!(out.len(), 0);

        // let (res, out) = comptime!(
        //     r#"
        //
        //
        // "#
        // );
    }

    #[test]
    fn hello_world() {
        let code = r#"Echo Hello, world!"#;

        let mut out = vec![];
        let res = exec_comptime(code, &mut out);
        assert!(res.is_ok());
        assert_eq!(out, vec!["Hello, world!".to_string()]);
    }
}
