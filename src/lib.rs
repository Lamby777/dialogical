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

mod comptime;
mod consts;
mod pages;
mod parser;

use parser::DgParser;

// Re-exports
pub use comptime::{Link, LinkKVPair, Result as ScriptResult};
pub use pages::Interaction;
pub use parser::Result as ParseResult;

type Error = Box<dyn std::error::Error>;

use once_cell::sync::OnceCell;
pub(crate) static SILENT: OnceCell<bool> = OnceCell::new();

macro_rules! log {
    ($($arg:tt)*) => {
        if !SILENT.get().unwrap_or(&false) {
            eprintln!($($arg)*);
        }
    };
}

/// Parse a single string into a `Vec<>` of interactions.
/// Uses the current directory as the parser path.
pub fn parse_all(data: &str) -> ParseResult<Vec<Interaction>> {
    // TODO no unwrap
    DgParser::new(std::env::current_dir().unwrap())
        .parse_all(data)
        .map(|v| v.to_vec())
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
