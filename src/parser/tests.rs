use super::*;

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

#[test]
fn parse_two_ix() {
    let data = include_dummy!("two_ix");

    let mut parser = DgParser::default();
    let parsed = parser.parse_all(data).unwrap();

    let expected = vec![
        Interaction {
            id: "First",
            pages: vec![
                Page {
                    metadata: PageMetadata::new_perm_double("Porky"),
                    content: "First page".to_owned(),
                },
                Page {
                    metadata: PageMetadata::new_perm_double("Ethan"),
                    content: "Second page".to_owned(),
                },
            ],
        },
        Interaction {
            id: "Second",
            pages: vec![
                Page {
                    metadata: PageMetadata::new_perm_double("Terra"),
                    content: "Third page".to_owned(),
                },
                Page {
                    metadata: PageMetadata::new_perm_double("Siva"),
                    content: "Fourth page".to_owned(),
                },
            ],
        },
    ];

    assert_eq!(parsed, expected);
}

#[test]
fn parse_pageonly() {
    let data = include_dummy!("pageonly");

    let mut parser = DgParser::default();
    let parsed = parser.parse_all(data).unwrap();

    let expected = Interaction {
        id: "PageOnly Test",
        pages: vec![
            Page {
                metadata: PageMetadata::new_perm_double("Mira"),
                content: "What's up?".to_owned(),
            },
            Page {
                metadata: PageMetadata {
                    speaker: Metadata::NoChange,
                    vox: Metadata::PageOnly("Ethan".to_owned()),
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

    assert_eq!(parsed, expected);
}

#[test]
fn parse_small_interaction() {
    // you're giving me some real small ix energy right now
    let data = include_dummy!("small_ix");

    let mut parser = DgParser::default();
    let parsed = parser.parse_all(data).unwrap();

    let expected = Interaction {
        id: "Test1",
        pages: vec![
            Page {
                metadata: PageMetadata::new_perm_double("Siva"),
                content: "First page".to_owned(),
            },
            Page {
                metadata: PageMetadata::new_perm_double("Terra"),
                content: "Second page\nWith more words".to_owned(),
            },
        ],
    };

    assert_eq!(parsed, expected);
}

#[test]
#[ignore = "too complicated for now"]
fn parse_one_ix_many_pages() {
    let data = include_dummy!("one_ix_many_pages");

    let mut parser = DgParser::default();
    let parsed = parser.parse_all(data).unwrap();

    let expected = Interaction {
        id: "Interaction",
        pages: vec![
            Page {
                metadata: PageMetadata::new_perm_double("Deez"),
                content: "When the words are sus".to_owned(),
            },
            Page {
                metadata: PageMetadata::new_perm_double("Gamer"),
                content: "Words go brrr".to_owned(),
            },
            Page {
                metadata: PageMetadata {
                    speaker: Metadata::NoChange,
                    vox: Metadata::NoChange,
                },
                content: "When the imposter is sus".to_owned(),
            },
            Page {
                metadata: PageMetadata::new_perm_double("Siva"),
                content: "Testing".to_owned(),
            },
        ],
    };

    assert_eq!(parsed, expected);
}
