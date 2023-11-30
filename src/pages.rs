//!
//! Data structures used by the parser
//!

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::comptime::ScriptError;
use crate::parser::{DialogueChoice, DialogueEnding};
use crate::ParseResult;

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
    Choices,
}

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("Encountered {0} while trying to parse interaction endings...")]
    MalformedEnding(String),

    #[error("Tried to push a mix of endings at: {0}")]
    MixedEndings(String),

    #[error("{0} is not a valid interaction ID")]
    InvalidID(String),

    #[error("Could not split {0}. Is this supposed to be metadata?")]
    NotMeta(String),

    #[error("{0} is not a valid metadata directive")]
    InvalidMeta(String),

    #[error("No interaction to set the ending for!")]
    EndingNoIX,

    #[error("No interaction to push the page into!")]
    PushPageNoIX,

    #[error("No interaction to push onto the parser's list!")]
    PushEmptyIX,

    #[error("Attempt to push a duplicate interaction")]
    PushDuplicateIX,

    #[error("Attempt to push a page after an ending in interaction")]
    PageAfterEnding,

    #[error("Failed while running comptime script")]
    Panic(ScriptError),
}

impl From<ScriptError> for ParseError {
    fn from(value: ScriptError) -> Self {
        Self::Panic(value)
    }
}

/// Represents a list of interactions, but with a HashMap
/// so you don't have to constantly filter through the list
pub type InteractionMap = HashMap<String, Interaction>;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Interaction {
    pub pages: Vec<Page>,
    pub ending: DialogueEnding,
}

impl Interaction {
    /// Will try to either push onto the list or start a list.
    /// It will error if there's currently a goto label.
    pub fn push_choice(&mut self, to_push: DialogueChoice) -> ParseResult<()> {
        use DialogueEnding::*;

        match &mut self.ending {
            // end, label, choices
            end @ End => {
                *end = Choices(vec![to_push]);
                Ok(())
            }

            Choices(ref mut list) => {
                list.push(to_push);
                Ok(())
            }

            Label(l) => Err(ParseError::MixedEndings(l.to_string())),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub metadata: PageMeta,
    pub content: String,
}

impl Page {
    pub fn from_content(content: String) -> Self {
        Self {
            content,
            metadata: PageMeta::default(),
        }
    }

    pub fn speaker(&self) -> &Speaker {
        &self.metadata.speaker.unwrap()
    }

    pub fn vox(&self) -> &str {
        &self.metadata.vox.unwrap()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Speaker {
    /// "Character's Name"
    Named(String),

    /// ""
    #[default]
    Narrator,

    /// "???"
    Unknown,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PageMeta {
    pub speaker: Metaline<Speaker>,
    pub vox: Metaline<String>,
}

impl PageMeta {
    pub fn nochange() -> Self {
        use Metaline::NoChange;

        Self {
            speaker: NoChange,
            vox: NoChange,
        }
    }
}

/// Represents a metadata directive
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Metaline<T> {
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

impl<T> Metaline<T> {
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
