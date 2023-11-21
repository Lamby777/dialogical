use thiserror::Error;

#[derive(Error, Debug)]
enum ScriptError {
    #[error("Panic for whatever dumb reason")]
    TestPanic,
}

type Result<T> = std::result::Result<T, ScriptError>;

fn exec_comptime(_code: &str) -> Result<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let code = r#"print("Hello, world!")"#;
        let res = exec_comptime(code);
        assert_eq!(res.unwrap(), "Hello, world!");
    }
}
