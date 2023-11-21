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

struct Script {
    content: String,
}

fn exec_comptime(script: &str, output: &mut Vec<String>) -> Result<()> {
    let lines = script.lines();

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let (key, val) = line.split_once(' ').ok_or(ScriptError::TestPanic)?;

        match key {
            "Echo" => {
                output.push(val.to_owned());
            }
            _ => {
                return Err(ScriptError::TestPanic);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blanked_out() {
        let code = r#"






        "#;
        let mut output = vec![];
        let res = exec_comptime(code, &mut output);
        assert!(res.is_ok());
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn hello_world() {
        let code = r#"Echo Hello, world!"#;

        let mut output = vec![];
        let res = exec_comptime(code, &mut output);
        assert!(res.is_ok());
        assert_eq!(output, vec!["Hello, world!".to_string()]);
    }
}
