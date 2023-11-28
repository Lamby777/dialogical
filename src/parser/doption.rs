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

    /// Function/Interaction to run when this option is picked
    pub label: Option<Label>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Label {
    /// Function label - name of a function to run
    Function(String),

    /// Interaction label - ID of an interaction to go to
    Goto(String),
}

impl Label {
    pub fn new_goto(id: &str) -> Self {
        Self::Goto(id.to_owned())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub enum DialogueEnding {
    /// Show a list of options for the user to pick from
    Options(Vec<DialogueOption>),

    /// Run a function or go to a different interaction
    ///
    /// How this is implemented in your game is up to you to decide...
    /// For Godot, this would be a GDScript function name...
    /// ...or maybe a signal? This tool won't make that decision for you.
    Label(Label),

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
