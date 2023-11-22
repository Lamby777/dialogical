use super::*;

macro_rules! comptime {
    ($code:expr) => {{
        let mut out = vec![];
        let mut links = vec![];
        let res = Script::from($code).execute(&mut out, &mut links);
        (res, out, links)
    }};
}

#[test]
fn quit_cmd() {
    let (res, out, _links) = comptime!(
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
    let (res, out, _links) = comptime!(
        r#"


        "#
    );
    assert!(res.is_ok());
    assert_eq!(out.len(), 0);
}

#[test]
fn hello_world() {
    let (res, out, _links) = comptime!(r#"Echo Hello, world!"#);
    assert!(res.is_ok());
    assert_eq!(out, vec!["Hello, world!".to_string()]);
}
