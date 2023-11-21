//!
//! Data structures used by the parser
//!

use thiserror::Error;

/// possible states the parser can be in
#[derive(Clone, Debug, Default)]
pub enum ParseState {
    /// Interaction-wide metadata not set yet, we're at the
    /// top of an interaction
    #[default]
    Start,

    /// Waiting to start a new interaction or comptime script
    Idle,

    /// Compile-time script block
    ComptimeScript,

    /// Directives written before a message
    /// Separated from the actual message by an empty line
    Metadata,

    /// Text content said by a character
    Message,

    /// Empty line after message, before the separator
    PostLine,
}

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("Encountered {0} instead of a separator while in PostLine state")]
    AfterPostline(String),

    #[error("{0} is not a valid interaction ID")]
    InvalidID(String),

    #[error("Could not split {0}. Is this supposed to be metadata?")]
    NotMeta(String),

    #[error("{0} is not a valid metadata directive")]
    InvalidMeta(String),
}

#[derive(Debug, PartialEq)]
pub struct Interaction<'a> {
    pub id: &'a str,
    pub pages: Vec<Page>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Page {
    pub metadata: PageMetadata,
    pub content: String,
}

impl Page {
    pub fn from_content(content: String) -> Self {
        Self {
            content,
            metadata: PageMetadata::default(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PageMetadata {
    pub speaker: Metadata<String>,
    pub vox: Metadata<String>,
}

impl PageMetadata {
    /// shorthand for permanent change of speaker and vox with same string
    /// good for writing quick unit tests
    pub fn new_perm_double(speaker: &str) -> Self {
        let meta = Metadata::Permanent(speaker.to_owned());

        Self {
            speaker: meta.clone(),
            vox: meta,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Metadata<T> {
    Permanent(T),
    PageOnly(T),
    NoChange,
}

impl<T> Default for Metadata<T> {
    fn default() -> Self {
        Self::NoChange
    }
}
