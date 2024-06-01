use std::path::PathBuf;

use super::*;
use crate::comptime::ScriptError;
use crate::pages::Metaline::*;
use crate::pages::PageMeta;
use crate::pages::Speaker::*;
use crate::parser::ParseError;
use crate::Label;

use map_macro::hash_map;
use pretty_assertions::assert_eq;

macro_rules! dummy_file {
    ($name:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/dummy_data/", $name, ".dg")
    };
}

macro_rules! dummy_parser {
    ($name:expr) => {{
        let path = PathBuf::from(dummy_file!($name));
        let path = path.canonicalize().unwrap();

        DgParser::new(path)
    }};
}

macro_rules! parse_dummy_err {
    ($name:expr) => {{
        let data = include_str!(dummy_file!($name));
        dummy_parser!($name).parse_all(data).unwrap_err()
    }};
}

macro_rules! parse_dummy {
    ($name:expr) => {{
        let data = include_str!(dummy_file!($name));
        dummy_parser!($name).parse_all(data).unwrap()
    }};
}

/// shorthand for permanent change of speaker and vox with same string
/// good for writing quick unit tests
macro_rules! meta_double {
    ($name:expr) => {
        PageMeta {
            speaker: Permanent(Named($name.to_owned())),
            vox: Permanent($name.to_owned()),
        }
    };
}

macro_rules! expected {
    ($hashkey:expr, $expected:tt) => {
        hash_map! {
            $hashkey.to_string() => expected!($expected)
        }
    };

    (small_ix) => {
        hash_map! {
            "Test1".to_string() => Interaction {
                pages: vec![
                    Page {
                        metadata: meta_double!("Siva"),
                        content: "First page".to_owned(),
                    },
                    Page {
                        metadata: meta_double!("Terra"),
                        content: "Second page\nWith more words".to_owned(),
                    },
                ],
                ending: DialogueEnding::End,
            }
        }
    };

    (link) => {
        hash_map! {
            "Link Test".to_string() => Interaction {
                pages: vec![
                    Page {
                        metadata: PageMeta {
                            speaker: Permanent(Named("Cherry".to_owned())),
                            vox: Permanent("Mira".to_owned()),
                        },
                        content: "Page 1".to_owned(),
                    },
                    Page {
                        metadata: PageMeta::nochange(),
                        content: "Page 2".to_owned(),
                    },
                    Page {
                        metadata: PageMeta::nochange(),
                        content: "Page 3".to_owned(),
                    },
                ],
                ending: DialogueEnding::End,
            }
        }
    };

    (link_between_ix) => {{
        let mut pt1 = expected!(link);

        let pt2 = hash_map! {
            "Link Test 2".to_string() => Interaction {
                pages: vec![
                    Page {
                        metadata: PageMeta {
                            speaker: Permanent(
                                Named("Cherry".to_owned()),
                            ),
                            vox: Permanent("Mira".to_owned()),
                        },
                        content: "Page 1, Second Interaction".to_owned(),
                    },
                ],
                ending: DialogueEnding::End,
            }
        };

        pt1.extend(pt2);

        pt1
    }};

    (two_ix) => {
        hash_map! {
            "First".to_string() => Interaction {
                pages: vec![
                    Page {
                        metadata: meta_double!("Porky"),
                        content: "First page".to_owned(),
                    },
                    Page {
                        metadata: meta_double!("Ethan"),
                        content: "Second page".to_owned(),
                    },
                ],
                ending: DialogueEnding::End,
            },
            "Second".to_string() => Interaction {
                pages: vec![
                    Page {
                        metadata: meta_double!("Terra"),
                        content: "Third page".to_owned(),
                    },
                    Page {
                        metadata: meta_double!("Siva"),
                        content: "Fourth page".to_owned(),
                    },
                ],
                ending: DialogueEnding::End,
            },
        }
    };

    (one_ix_many_pages) => {
        hash_map! {
            "Interaction".to_string() => Interaction {
                pages: vec![
                    Page {
                        metadata: meta_double!("Deez"),
                        content: "When the words are sus".to_owned(),
                    },
                    Page {
                        metadata: PageMeta {
                            speaker: Permanent(Named("Gamer".to_owned())),
                            vox: NoChange,
                        },
                        content: "Words go brrr".to_owned(),
                    },
                    Page {
                        metadata: PageMeta::nochange(),
                        content: "When the imposter is sus".to_owned(),
                    },
                    Page {
                        metadata: meta_double!("Siva"),
                        content: "Testing".to_owned(),
                    },
                ],
                ending: DialogueEnding::End,
            }
        }
    };

    (import_others) => {{
        let mut map = expected!(small_ix);
        map.extend(expected!(link));
        map.extend(expected!(two_ix));
        map.extend(expected!(one_ix_many_pages));

        map
    }};

    (import_sub) => {{
        let mut map = expected!(import_others);
        map.extend(expected!(rodrick));

        map
    }};

    (rodrick) => {{
        let first_meta = PageMeta {
            speaker: Permanent(Named("Rodrick Sign Co.".to_owned())),
            vox: Permanent("Default".to_owned()),
        };

        hash_map! {
            "RodrickSign".to_string() => Interaction {
                pages: vec![
                    Page {
                        metadata: first_meta.clone(),
                        content: "So... you're reading a sign, eh?".to_owned(),
                    },
                    Page {
                        metadata: PageMeta::nochange(),
                        content: "Well...".to_owned(),
                    },
                    Page {
                        metadata: PageMeta::nochange(),
                        content: "Are you smart?".to_owned(),
                    },
                ],
                ending: DialogueEnding::Choices(vec![
                    DialogueChoice {
                        text: "Nope".to_string(),
                        label: Some(Label::new_goto("RodrickSign_Nope")),
                    },
                    DialogueChoice {
                        text: "Definitely not".to_string(),
                        label: Some(Label::new_goto("RodrickSign_DefNot")),
                    },
                ]),
            },
            "RodrickSign_Nope".to_string() => Interaction {
                pages: vec![Page {
                    metadata: first_meta.clone(),
                    content: "Yeah, I didn't think so.".to_owned(),
                }],
                ending: DialogueEnding::Label(Label::new_goto("RodrickSign_Exit")),
            },
            "RodrickSign_DefNot".to_string() => Interaction {
                pages: vec![Page {
                    metadata: first_meta.clone(),
                    content: "Yeah, I definitely didn't think so.".to_owned(),
                }],
                ending: DialogueEnding::Label(Label::new_goto("RodrickSign_Exit")),
            },
            "RodrickSign_Exit".to_string() => Interaction {
                pages: vec![Page {
                    metadata: first_meta.clone(),
                    content: "Come back when you're smart.".to_owned(),
                }],
                ending: DialogueEnding::Label(Label::new_fn("Exit")),
            },
        }
    }};
}

#[test]
fn parse_rodrick_sign() {
    let parsed = parse_dummy!("rodrick");
    assert_eq!(parsed, expected!(rodrick));
}

#[test]
fn import_subimports() {
    let parsed = parse_dummy!("import_sub");
    assert_eq!(parsed, expected!(import_sub));
}

#[test]
fn import_others() {
    let parsed = parse_dummy!("import");
    assert_eq!(parsed, expected!(import_others));
}

#[test]
fn link_name_to_vox() {
    let parsed = parse_dummy!("link");
    assert_eq!(parsed, expected!(link));
}

#[test]
fn link_between_ix() {
    let parsed = parse_dummy!("link_between_ix");
    assert_eq!(parsed, expected!(link_between_ix));
}

#[test]
fn parse_two_ix() {
    let parsed = parse_dummy!("two_ix");
    assert_eq!(parsed, expected!(two_ix));
}

#[test]
fn parse_small_interaction() {
    // you're giving me some real small ix energy right now
    let parsed = parse_dummy!("small_ix");
    assert_eq!(parsed, expected!(small_ix));
}

#[test]
fn parse_one_ix_many_pages() {
    let parsed = parse_dummy!("one_ix_many_pages");
    assert_eq!(parsed, expected!(one_ix_many_pages));
}

#[test]
fn page_after_end() {
    let parsed = parse_dummy_err!("vsauce");
    assert_eq!(parsed, ParseError::PageAfterEnding);
}

#[test]
fn double_link() {
    let parsed = parse_dummy_err!("agent_link");
    assert_eq!(parsed, ParseError::Panic(ScriptError::DoubleLink));
}

#[test]
fn dupe_ix_ids() {
    let parsed = parse_dummy_err!("dupe_ix");
    assert_eq!(parsed, ParseError::PushDuplicateIX);
}

#[test]
fn empty_ix_before_import() {
    let parsed = parse_dummy!("pets/main");
    [
        "This interaction ID should not prevent the import",
        "Rodrick Sign #1",
        "Rodrick Sign #1 >> Nope",
        "Rodrick Sign #1 >> DefNot",
        "Rodrick Sign #1 >> Exit",
    ]
    .iter()
    .for_each(|v| {
        assert!(parsed.contains_key(*v), "Missing interaction {}", v);
    })
}

#[test]
fn newline_tricks() {
    let parsed = parse_dummy!("newlines");
    let pages = parsed.get("Newline Tricks").unwrap().pages.as_slice();

    let verse = vec![
        "Buffer ended, you were not streamin',",
        "Try concat to it, try to parse through it.",
        "You wuh nah thinkin' that uh I would log to it,",
        "They be loggin' somewhat and uh I would log stupid.",
        "Take the next ticket, take the next string.",
        "Why would I do it, anyone thinkin' that uh--",
    ];

    let [spaces, newlines, _rest @ ..] = pages else {
        panic!("Parsed incorrect number of pages");
    };

    // newlines are spaces, unless the line ends in \n
    assert_eq!(spaces.content, verse.join(" "));
    assert_eq!(newlines.content, verse.join("\n"));
}

#[test]
fn unlink_name_to_vox() {
    let parsed = parse_dummy!("unlink");
    let expected = hash_map! {
        "Unlink Test".to_string() => Interaction {
            pages: vec![
                Page {
                    metadata: meta_double!("Mira"),
                    content: "Page 1".to_owned(),
                },
                Page {
                    metadata: PageMeta::nochange(),
                    content: "Page 2".to_owned(),
                },
                Page {
                    metadata: meta_double!("Dylan"),
                    content: "Page 3".to_owned(),
                },
                Page {
                    metadata: PageMeta {
                        speaker: Permanent(Named("Mira".to_owned())),
                        vox: NoChange,
                    },
                    content: "Page 4".to_owned(),
                },
                Page {
                    metadata: meta_double!("Dylan"),
                    content: "Page 5".to_owned(),
                },
            ],
            ending: DialogueEnding::End,
        }
    };

    assert_eq!(parsed, expected);
}

#[test]
fn parse_filter_empties() {
    let parsed = parse_dummy!("empties");
    let expected = hash_map! {
        "Empties Test".to_string() => Interaction {
        pages: vec![
            Page {
                metadata: meta_double!("Siva"),
                content: "So uhh... what's your name?".to_owned(),
            },
            Page {
                metadata: meta_double!("L'yembo"),
                content: "...".to_owned(),
            },
            Page {
                metadata: PageMeta::nochange(),
                content: "--- ---".to_owned(),
            },
            Page {
                metadata: PageMeta::nochange(),
                content: "*runs away*".to_owned(),
            },
        ],
        ending: DialogueEnding::End,
    }};

    assert_eq!(parsed, expected);
}

#[test]
#[rustfmt::skip] // ffs stop formatting raw strings
fn parse_pageonly() {
    let parsed = parse_dummy!("pageonly");
    let expected = hash_map! {
        "PageOnly Test".to_string() => Interaction {
        pages: vec![
            Page {
                metadata: meta_double!("Mira"),
                content: "What's up?".to_owned(),
            },
            Page {
                metadata: PageMeta {
                    speaker: NoChange,
                    vox: PageOnly("Ethan".to_owned()),
                },

                content: "Nothing much...".to_owned(),
            },
            Page {
                metadata: PageMeta::default(),
                content: r#"Alright, why am I talking to myself?
Who's making me do this?"#
                    .to_owned(),
            },
        ],
        ending: DialogueEnding::End,
    }
    };

    assert_eq!(parsed, expected);
}
