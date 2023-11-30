//!
//! Type definitions for the `Execute` and `Import` directives.
//!
//! IMPORTANT: `Execute` is NOT for importing interactions from
//! other files! It just executes the contents of the file as-is
//! in a separate execution environment. If you want to import
//! interactions from another file, PLEASE use the `Import`
//! directive instead.
//!
//! TODO unit test if that claim is actually true ^^^ lol
//!

use std::fs::File;
use std::io;
use std::path::PathBuf;

use super::{Result, ScriptError};
use crate::parser::DgParser;
use crate::{InteractionMap, ParseResult};

/// Used for `Execute` and `Import` directives.
#[derive(Clone, Debug)]
pub struct ScriptPath(pub PathBuf);

impl ScriptPath {
    /// Create new ScriptPath by resolving one and appending
    /// a new path onto it
    pub fn make_append(&self, path: PathBuf) -> Self {
        Self(self.0.parent().unwrap().join(path))
    }

    /// Get the contents of the script at the path.
    /// Used by the `Execute` directive.
    pub fn read(&self) -> Result<String> {
        File::open(&self.0)
            .and_then(|file| io::read_to_string(file))
            .map_err(|_| ScriptError::FileOpen(self.0.clone()))
    }

    /// Run a second parser instance on the script at the path.
    /// Used by the `Import` directive.
    pub fn parse(&self) -> ParseResult<InteractionMap> {
        let contents = self.read()?;

        let mut parser = DgParser::new(self.0.clone());
        parser.parse_all(&contents)
    }
}
