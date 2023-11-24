//!
//! Type definitions for the `Include` and `Import` directives.
//!
//! IMPORTANT: `Include` is NOT for importing interactions from
//! other files! It just includes the contents of the file as-is,
//! like a C preprocessor. If you want to import interactions from
//! another file, PLEASE use the `Import` directive instead.
//!
//! This is only good for including stuff like common `Link`
//! commands that you don't want to have to keep typing out at the
//! top of every single dialogue file you write.
//!

use std::path::PathBuf;

pub struct Include(pub PathBuf);
