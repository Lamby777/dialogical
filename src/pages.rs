//!
//! Data structures used by the parser
//!

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::comptime::ScriptError;

/// possible states the parser can be in
#[derive(Clone, Debug, Default)]
pub enum ParseState {
    /// Compile-time script block
    ///
    /// Also carries the previous parser state so it
    /// can return to it once the script is done
    ComptimeScript(Box<ParseState>),

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

    #[error("No interaction to push the page into!")]
    PushPageNoIX,

    #[error("No interaction to push onto the parser's list!")]
    PushEmptyIX,

    #[error("Failed while running comptime script")]
    Panic(ScriptError),
}

impl From<ScriptError> for ParseError {
    fn from(value: ScriptError) -> Self {
        Self::Panic(value)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Interaction {
    // TODO use &'static str
    pub id: String,
    pub pages: Vec<Page>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Speaker {
    /// "Character's Name"
    Named(String),

    /// ""
    #[default]
    Narrator,

    /// TODO command for this mode
    /// "???"
    Unknown,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PageMetadata {
    pub speaker: Metadata<Speaker>,
    pub vox: Metadata<String>,
}

impl PageMetadata {
    pub fn nochange() -> Self {
        use Metadata::NoChange;

        Self {
            speaker: NoChange,
            vox: NoChange,
        }
    }
}

/// Represents a metadata directive
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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
    pub fn new(val: T, pageonly: bool) -> Self {
        if pageonly {
            Self::PageOnly(val)
        } else {
            Self::Permanent(val)
        }
    }

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
