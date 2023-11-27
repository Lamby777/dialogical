//!
//! Module for type definitions related to dialogue options
//! and anything else that happens at the end of an interaction
//!

use serde::{Deserialize, Serialize};

/// One option in a list of dialogue options
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DialogueOption {
    /// Text displayed for the choice
    /// Not necessarily unique...
    pub text: String,

    /// Interaction ID to jump to if this option is selected
    pub goto_label: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub enum DialogueEnding {
    Options(Vec<DialogueOption>),
    Goto(String),

    #[default]
    End,
}

impl DialogueEnding {
    pub fn append_option(&mut self, option: DialogueOption) {
        match self {
            DialogueEnding::Options(ref mut options) => {
                options.push(option);
            }

            _ => panic!("Tried to append option to non-options ending"),
        }
    }
}
