use super::*;
use crate::pages::PageMetadata;
use crate::parse_all;

use crate::pages::Metadata::*;
use crate::pages::Speaker::*;

macro_rules! dummy_file {
    ($name:expr) => {
        concat!("../../dummy_data/", $name, ".dg")
    };
}

macro_rules! include_dummy {
    ($name:expr) => {
        include_str!(dummy_file!($name))
    };
}

macro_rules! parse_dummy {
    ($name:expr) => {{
        let data = include_dummy!($name);
        parse_all(data).unwrap()
    }};
}

/// shorthand for permanent change of speaker and vox with same string
/// good for writing quick unit tests
macro_rules! meta_double {
    ($name:expr) => {
        PageMetadata {
            speaker: Permanent(Named($name.to_owned())),
            vox: Permanent($name.to_owned()),
        }
    };
}

macro_rules! expected {
    (small_ix) => {
        Interaction {
            id: "Test1".to_string(),
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
        }
    };

    (link) => {
        Interaction {
            id: "Link Test".to_string(),
            pages: vec![
                Page {
                    metadata: PageMetadata {
                        speaker: Permanent(Named("Cherry".to_owned())),
                        vox: Permanent("Mira".to_owned()),
                    },
                    content: "Page 1".to_owned(),
                },
                Page {
                    metadata: PageMetadata::nochange(),
                    content: "Page 2".to_owned(),
                },
                Page {
                    metadata: PageMetadata::nochange(),
                    content: "Page 3".to_owned(),
                },
            ],
        }
    };

    (two_ix) => {
        vec![
            Interaction {
                id: "First".to_string(),
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
            },
            Interaction {
                id: "Second".to_string(),
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
            },
        ]
    };

    (one_ix_many_pages) => {
        Interaction {
            id: "Interaction".to_string(),
            pages: vec![
                Page {
                    metadata: meta_double!("Deez"),
                    content: "When the words are sus".to_owned(),
                },
                Page {
                    metadata: PageMetadata {
                        speaker: Permanent(Named("Gamer".to_owned())),
                        vox: NoChange,
                    },
                    content: "Words go brrr".to_owned(),
                },
                Page {
                    metadata: PageMetadata::nochange(),
                    content: "When the imposter is sus".to_owned(),
                },
                Page {
                    metadata: meta_double!("Siva"),
                    content: "Testing".to_owned(),
                },
            ],
        }
    };
}

#[test]
fn import() {
    println!("{:?}", crate::ENTRY_PATH.get());

    crate::ENTRY_PATH.set("dummy_data/import".into()).unwrap();
    let parsed = parse_dummy!("import");

    let two_ix = expected!(two_ix);
    let expected = vec![
        expected!(small_ix),
        expected!(link),
        two_ix[0].clone(),
        two_ix[1].clone(),
        expected!(one_ix_many_pages),
    ];

    assert_eq!(parsed, expected);
}

#[test]
fn unlink_name_to_vox() {
    let parsed = parse_dummy!("unlink");
    let expected = Interaction {
        id: "Unlink Test".to_string(),
        pages: vec![
            Page {
                metadata: meta_double!("Mira"),
                content: "Page 1".to_owned(),
            },
            Page {
                metadata: PageMetadata::nochange(),
                content: "Page 2".to_owned(),
            },
            Page {
                metadata: meta_double!("Dylan"),
                content: "Page 3".to_owned(),
            },
            Page {
                metadata: PageMetadata {
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
    };

    assert_eq!(parsed, vec![expected]);
}

#[test]
fn link_name_to_vox() {
    let parsed = parse_dummy!("link");
    assert_eq!(parsed, vec![expected!(link)]);
}

#[test]
fn parse_filter_empties() {
    let parsed = parse_dummy!("empties");
    let expected = vec![Interaction {
        id: "Empties Test".to_string(),
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
                metadata: PageMetadata::nochange(),
                content: "---\n---".to_owned(),
            },
            Page {
                metadata: PageMetadata::nochange(),
                content: "*runs away*".to_owned(),
            },
        ],
    }];

    assert_eq!(parsed, expected);
}

#[test]
fn parse_two_ix() {
    let parsed = parse_dummy!("two_ix");
    assert_eq!(parsed, expected!(two_ix));
}

#[test]
fn parse_pageonly() {
    let parsed = parse_dummy!("pageonly");
    let expected = Interaction {
        id: "PageOnly Test".to_string(),
        pages: vec![
            Page {
                metadata: meta_double!("Mira"),
                content: "What's up?".to_owned(),
            },
            Page {
                metadata: PageMetadata {
                    speaker: NoChange,
                    vox: PageOnly("Ethan".to_owned()),
                },

                content: "Nothing much...".to_owned(),
            },
            Page {
                metadata: PageMetadata::default(),
                content: r#"Alright, why am I talking to myself?
Who's making me do this?"#
                    .to_owned(),
            },
        ],
    };

    assert_eq!(parsed, vec![expected]);
}

#[test]
fn parse_small_interaction() {
    // you're giving me some real small ix energy right now
    let parsed = parse_dummy!("small_ix");
    assert_eq!(parsed, vec![expected!(small_ix)]);
}

#[test]
fn parse_one_ix_many_pages() {
    let parsed = parse_dummy!("one_ix_many_pages");
    assert_eq!(parsed, vec![expected!(one_ix_many_pages)]);
}
