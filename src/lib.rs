//!
//! literally rewriting it in rust because i got tired
//! of trying to make modules work properly instead of
//! being able to write actual code
//!
//!  - &Cherry, 11/20/2023
//!

#![feature(if_let_guard)]

use clap::Parser;

use std::fs::File;
use std::io::{self, Read, Write};
use std::sync::OnceLock;

mod comptime;
mod consts;
mod pages;
mod parser;

use comptime::{Link, LinkKVPair};
use parser::{DgParser, Result as ParseResult};

// Re-exports
pub use pages::{Interaction, Metaline, Page, PageMeta, Speaker};
pub use parser::{DialogueChoice, DialogueEnding, Label};

type Error = Box<dyn std::error::Error>;

pub(crate) static SILENT: OnceLock<bool> = OnceLock::new();

macro_rules! log {
    ($($arg:tt)*) => {
        if !SILENT.get().unwrap_or(&false) {
            eprintln!($($arg)*);
        }
    };
}

pub fn deserialize(data: &[u8]) -> Result<Vec<Interaction>, Error> {
    match bincode::deserialize(data) {
        Ok(v) => {
            log!("Deserialized successfully!");
            Ok(v)
        }

        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

pub fn main(args: Args) -> Result<(), Error> {
    SILENT.set(args.silent).unwrap();

    // TODO error handling for file rw
    let input_stream: Box<dyn Read> = match args.file {
        Some(ref file) => Box::new(File::open(file).unwrap()),
        None => Box::new(io::stdin()),
    };

    let mut output_stream: Box<dyn Write> = match args.output {
        Some(file) => Box::new(File::create(file).unwrap()),
        None => Box::new(io::stdout()),
    };

    log!("Reading...");
    let data = io::read_to_string(input_stream)?;

    log!("Parsing...");
    // if stdin, the path is the cwd
    let path = if let Some(ref path) = args.file {
        std::path::PathBuf::from(path)
    } else {
        std::env::current_dir()?
    };

    let mut parser = DgParser::new(path);
    let res = parser.parse_all(&data)?;

    log!("Serializing...");
    let res = bincode::serialize(&res)?;

    log!("Writing...");
    output_stream.write(&res)?;

    log!("Done!");
    Ok(())
}

#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
#[command(author, version, about)]
/// P/E/T/S Dialogue Compiler
pub struct Args {
    /// The output file, or stdout if not specified
    #[arg(short, long)]
    output: Option<String>,

    /// The input file, or stdin if not specified
    file: Option<String>,

    /// Silences progress "info" stderr messages.
    #[arg(short, long)]
    silent: bool,
}
