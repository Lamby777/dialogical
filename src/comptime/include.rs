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
#[derive(Clone)]
pub struct ScriptPath {
    pub to: PathBuf,
    pub from: PathBuf,
}

impl Default for ScriptPath {
    fn default() -> Self {
        Self {
            from: std::env::current_dir().unwrap(),
            to: PathBuf::default(),
        }
    }
}

impl ScriptPath {
    /// Create new ScriptPath by resolving one and appending
    /// a new path onto it
    pub fn make_append(&self, path: &str) -> Self {
        let new_from = self.resolve();
        Self {
            from: new_from,
            to: PathBuf::from(path),
        }
    }

    /// Resolve the path by combining `from` and `to`.
    pub fn resolve(&self) -> PathBuf {
        self.from.join(&self.to)
    }

    /// Get the contents of the script at the path.
    /// Used by the `Execute` directive.
    pub fn read(&self) -> Result<String> {
        let path = self.resolve();

        File::open(path.clone())
            .and_then(|file| io::read_to_string(file))
            .map_err(|_| ScriptError::FileOpen(path))
    }

    /// Run a second parser instance on the script at the path.
    /// Used by the `Import` directive.
    pub fn parse(&self) -> ParseResult<Vec<Interaction>> {
        let contents = self.read()?;
        let interactions = parse_all(&contents)?;
        Ok(interactions)
    }
}
