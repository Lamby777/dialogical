use super::*;

#[test]
fn parse_small_interaction() {
    // you're giving me some real small ix energy right now
    let data = include_str!("../../dummy_data/small_ix.dg");

    let mut parser = DgParser::default();
    let parsed = parser.parse(data).unwrap();

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
fn parse_one_interaction_many_pages() {
    let data = include_str!("../../dummy_data/one_ix_many_pages.dg");

    let mut parser = DgParser::default();
    let parsed = parser.parse(data).unwrap();

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
