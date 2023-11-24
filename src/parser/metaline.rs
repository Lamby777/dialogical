use crate::comptime::LinkKVPair;
use crate::consts::COMPTIME_BORDER;
use crate::pages::{Metadata, ParseError, ParseState, Speaker};

use super::{DgParser, Result};

/// split into first "word" and the rest
fn split_first_whitespace(full: &str) -> Result<(&str, &str)> {
    full.split_once(char::is_whitespace)
        .ok_or(ParseError::NotMeta(full.to_string()))
        .map(|(k, v)| (k, v.trim_start()))
}

/// parse a comptime scripting block
pub fn parse(parser: &mut DgParser, line: &str) -> Result<()> {
    if line == COMPTIME_BORDER {
        // comptime script inside a comptime script is 100% a parsing error
        debug_assert!(!matches!(parser.state, ParseState::ComptimeScript(_)));

        parser.state = ParseState::ComptimeScript(Box::new(parser.state.clone()));
        return Ok(());
    }

    let (kv, pageonly) = {
        // everything after the space is the value
        let kv = split_first_whitespace(line)?;

        // ...unless the key is PageOnly, in which case we
        // repeat the process again
        if kv.0 == "PageOnly" {
            (split_first_whitespace(kv.1)?, true)
        } else {
            (kv, false)
        }
    };

    match kv.0 {
        "NARRATOR" | "SOMEONE" => {
            let speaker = if kv.0 == "NARRATOR" {
                Speaker::Narrator
            } else {
                Speaker::Unknown
            };

            parser.page.metadata.speaker = Metadata::new(speaker, pageonly);
            return Ok(());
        }

        "%" if !pageonly => {
            return parser.set_ix_id(kv.1);
        }

        _ => {}
    }

    // the pair + any pairs linked using the `Link` directive
    let pair = LinkKVPair::from_tuple(kv);
    let links = parser.get_links_for(pair);
    let mapped = links.iter().map(|v| (v.0.as_str(), v.1.as_str()));
    let kvpairs = std::iter::once(kv).chain(mapped);

    for (key, val) in kvpairs {
        match key {
            "NAME" => {
                let name = Speaker::Named(val.to_owned());
                parser.page.metadata.speaker = Metadata::new(name, pageonly)
            }

            "VOX" => parser.page.metadata.vox = Metadata::new(val.to_owned(), pageonly),

            _ => {
                return Err(ParseError::InvalidMeta(line.to_string()));
            }
        };
    }

    Ok(())
}
