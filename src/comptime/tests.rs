use super::*;

macro_rules! comptime {
    ($code:expr) => {{
        let mut out = ScriptContext::default();
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
    assert_eq!(out.logs(), vec!["I just wanted to say...".to_owned()]);
}

#[test]
fn blanked_out() {
    let (res, out) = comptime!(
        r#"


        "#
    );
    assert!(res.is_ok());
    assert_eq!(out.iter_logs().count(), 0);
}

#[test]
fn hello_world() {
    let (res, out) = comptime!(r#"Echo Hello, world!"#);
    assert!(res.is_ok());
    assert_eq!(out.logs(), vec!["Hello, world!".to_string()]);
}
