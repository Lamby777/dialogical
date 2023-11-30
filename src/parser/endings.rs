//!
//! Module for type definitions related to dialogue choices
//! and anything else that happens at the end of an interaction
//!

use serde::{Deserialize, Serialize};

use std::fmt;

use crate::consts::*;
use crate::pages::{ParseError, ParseState};
use crate::{DgParser, ParseResult};

/// One choice in a list of dialogue choices
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DialogueChoice {
    /// Text displayed for the choice
    /// Not necessarily unique...
    pub text: String,

    /// Function/Interaction to run when this choice is picked
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

    pub fn new_fn(name: &str) -> Self {
        Self::Function(name.to_owned())
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Function(name) => write!(f, "{}", name),
            Self::Goto(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub enum DialogueEnding {
    /// Show a list of choices for the user to pick from
    Choices(Vec<DialogueChoice>),

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
    pub fn append_choice(&mut self, choice: DialogueChoice) -> ParseResult<()> {
        match self {
            DialogueEnding::Choices(ref mut choices) => {
                choices.push(choice);
            }

            DialogueEnding::End => {
                *self = DialogueEnding::Choices(vec![choice]);
            }

            _ => return Err(ParseError::MixedEndings(choice.text.clone())),
        }
        Ok(())
    }
}

pub fn parse(parser: &mut DgParser, line: &str) -> ParseResult<()> {
    // skip empty lines
    if line.is_empty() {
        return Ok(());
    }

    // if the line is a separator and we're not in the
    // middle of parsing an ending, then we're done.
    //
    // push the page and move on.
    if line == SEPARATOR {
        parser.push_page()?;
        parser.state = ParseState::Metadata;
        return Ok(());
    }

    // split the line into prefix (>, @, $) and the rest
    let (first_ch, rest) = {
        let mut it = line.chars();

        let first_ch = it
            .next()
            .ok_or(ParseError::MalformedEnding(line.to_owned()))?;

        it.next(); // skip the space
        (first_ch, it.as_str())
    };

    let ix = parser
        .interaction
        .as_mut()
        .ok_or(ParseError::PushPageNoIX)?;
    match first_ch {
        PREFIX_CHOICE => {
            // parse a choice
            let choice = DialogueChoice {
                text: rest.to_owned(),
                label: None,
            };

            ix.ending.append_choice(choice)?;
        }

        // if label, then add a label to the previous choice
        // OR set the label of the entire interaction if there is none
        // if one exists, error out.
        _ => {
            let variant = match first_ch {
                PREFIX_GOTO_LABEL => Label::new_goto,
                PREFIX_GOTO_FN => Label::new_fn,
                _ => return Err(ParseError::MalformedEnding(line.to_owned())),
            };

            let label = variant(rest);
            match ix.ending {
                DialogueEnding::Choices(ref mut choices) => {
                    let choice = choices
                        .last_mut()
                        .ok_or_else(|| ParseError::MalformedEnding(line.to_owned()))?;

                    if choice.label.is_some() {
                        return Err(ParseError::MixedEndings(line.to_owned()));
                    }

                    choice.label = Some(label);
                }

                DialogueEnding::Label(_) => {
                    return Err(ParseError::MixedEndings(line.to_owned()));
                }

                DialogueEnding::End => {
                    ix.ending = DialogueEnding::Label(label);
                }
            }
        }
    }

    Ok(())
}
