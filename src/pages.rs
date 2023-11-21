//!
//! Data structures used by the parser
//!

use thiserror::Error;

/// possible states the parser can be in
#[derive(Clone, Debug, Default)]
pub enum ParseState {
    /// Compile-time script block
    ComptimeScript,

    /// Directives written before a message
    /// Separated from the actual message by an empty line
    #[default]
    Metadata,

    /// Text content said by a character
    Message,

    /// Empty line after message, before the separator
    /// TODO change this to options and parse 'em into a vec
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

    #[error("No interaction to push")]
    PushEmpty,
}

#[derive(Debug, PartialEq)]
pub struct Interaction {
    // TODO use &'static str
    pub id: String,
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

/// Represents a metadata directive
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Metadata<T> {
    /// Any change is implicitly permanent
    /// Example:
    /// `NAME &Cherry`
    Permanent(T),

    /// PageOnly changes must be explicitly marked as such
    /// Example:
    /// `PageOnly NAME &Cherry`
    PageOnly(T),

    /// If no change is specified, the variant `NoChange` is used
    #[default]
    NoChange,
}

impl<T> Metadata<T> {
    /// None if NoChange
    ///
    /// I guess you can think of Metadata like
    /// Option<T> but with 3 variants instead of 2
    pub fn try_unwrap(&self) -> Option<&T> {
        match self {
            Self::Permanent(val) | Self::PageOnly(val) => Some(val),
            Self::NoChange => None,
        }
    }

    /// shorthand to "just get it done"
    pub fn unwrap(&self) -> &T {
        self.try_unwrap().unwrap()
    }
}
