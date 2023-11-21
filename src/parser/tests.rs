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

/// shorthand for permanent change of speaker and vox with same string
/// good for writing quick unit tests
fn meta_double(name: &str) -> PageMetadata {
    use Metadata::Permanent;

    PageMetadata {
        speaker: Permanent(Speaker::Name(name.to_owned())),
        vox: Permanent(name.to_owned()),
    }
}

#[test]
fn parse_filter_empties() {
    let data = include_dummy!("empties");

    let mut parser = DgParser::default();
    let parsed = parser.parse_all(data).unwrap();

    let expected = vec![Interaction {
        id: "Empties Test".to_string(),
        pages: vec![
            Page {
                metadata: meta_double("Siva"),
                content: "So uhh... what's your name?".to_owned(),
            },
            Page {
                metadata: meta_double("L'yembo"),
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

fn parse_two_ix() {
    let data = include_dummy!("two_ix");

    let mut parser = DgParser::default();
    let parsed = parser.parse_all(data).unwrap();

    let expected = vec![
        Interaction {
            id: "First".to_string(),
            pages: vec![
                Page {
                    metadata: meta_double("Porky"),
                    content: "First page".to_owned(),
                },
                Page {
                    metadata: meta_double("Ethan"),
                    content: "Second page".to_owned(),
                },
            ],
        },
        Interaction {
            id: "Second".to_string(),
            pages: vec![
                Page {
                    metadata: meta_double("Terra"),
                    content: "Third page".to_owned(),
                },
                Page {
                    metadata: meta_double("Siva"),
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
        id: "PageOnly Test".to_string(),
        pages: vec![
            Page {
                metadata: meta_double("Mira"),
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

    assert_eq!(parsed, vec![expected]);
}

#[test]
fn parse_small_interaction() {
    // you're giving me some real small ix energy right now
    let data = include_dummy!("small_ix");

    let mut parser = DgParser::default();
    let parsed = parser.parse_all(data).unwrap();

    let expected = Interaction {
        id: "Test1".to_string(),
        pages: vec![
            Page {
                metadata: meta_double("Siva"),
                content: "First page".to_owned(),
            },
            Page {
                metadata: meta_double("Terra"),
                content: "Second page\nWith more words".to_owned(),
            },
        ],
    };

    assert_eq!(parsed, vec![expected]);
}

#[test]
#[ignore = "too complicated for now"]
fn parse_one_ix_many_pages() {
    let data = include_dummy!("one_ix_many_pages");

    let mut parser = DgParser::default();
    let parsed = parser.parse_all(data).unwrap();

    let expected = Interaction {
        id: "Interaction".to_string(),
        pages: vec![
            Page {
                metadata: meta_double("Deez"),
                content: "When the words are sus".to_owned(),
            },
            Page {
                metadata: meta_double("Gamer"),
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
                metadata: meta_double("Siva"),
                content: "Testing".to_owned(),
            },
        ],
    };

    assert_eq!(parsed, vec![expected]);
}
