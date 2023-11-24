//!
//! Type definitions for the `Include` and `Import` directives.
//!
//! IMPORTANT: `Include` is NOT for importing interactions from
//! other files! It just includes the contents of the file as-is,
//! like a C preprocessor. If you want to import interactions from
//! another file, PLEASE use the `Import` directive instead.
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

/// Used for `Include` and `Import` directives.
pub struct ScriptPath(pub PathBuf);

impl ScriptPath {
    /// Get the contents of the script at the path.
    /// Used by the `Include` directive.
    fn resolve(&self) -> Result<String> {
        let file = File::open(&self.0).map_err(|_| ScriptError::FileOpenError(self.0.clone()))?;
        let contents =
            io::read_to_string(file).map_err(|_| ScriptError::FileOpenError(self.0.clone()))?;

        Ok(contents)
    }

    /// Run a second parser instance on the script at the path.
    /// Used by the `Import` directive.
    fn parse(&self) -> ParseResult<Vec<Interaction>> {
        let contents = self.resolve()?;
        let interactions = parse_all(&contents)?;
        Ok(interactions)
    }
}
