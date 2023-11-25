//!
//! Type definitions for the `Execute` and `Import` directives.
//!
//! IMPORTANT: `Execute` is NOT for importing interactions from
//! other files! It just executes the contents of the file as-is
//! in a separate execution environment. If you want to import
//! interactions from another file, PLEASE use the `Import`
//! directive instead.
//!
//! `Inlude` is only good for including stuff like common `Link`
//! commands that you don't want to have to keep typing out at the
//! top of every single dialogue file you write.
//!

use std::fs::File;
use std::io;
use std::path::PathBuf;

use super::{Result, ScriptError};
use crate::{parse_all, Interaction, ParseResult};

/// Used for `Execute` and `Import` directives.
pub struct ScriptPath {
    pub to: PathBuf,
    pub from: PathBuf,
}

impl ScriptPath {
    /// Get the contents of the script at the path.
    /// Used by the `Execute` directive.
    pub fn resolve(&self) -> Result<String> {
        File::open(&self.to)
            .and_then(|file| io::read_to_string(file))
            .map_err(|_| ScriptError::FileOpen(self.to.clone()))
    }

    /// Run a second parser instance on the script at the path.
    /// Used by the `Import` directive.
    pub fn parse(&self) -> ParseResult<Vec<Interaction>> {
        let contents = self.resolve()?;
        let interactions = parse_all(&contents)?;
        Ok(interactions)
    }
}
