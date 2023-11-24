//!
//! literally rewriting it in rust because i got tired
//! of trying to make modules work properly instead of
//! being able to write actual code
//!
//!  - &Cherry, 11/20/2023
//!

#![feature(if_let_guard)]

use bincode::serialize;
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

macro_rules! log {
    ($silent:expr, $($arg:tt)*) => {
        if !$silent {
            eprintln!($($arg)*);
        }
    };
}

/// Parse a single string into a `Vec<>` of interactions.
pub fn parse_all(data: &str) -> ParseResult<Vec<Interaction>> {
    DgParser::default().parse_all(data).map(|v| v.to_vec())
}

pub fn main(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let silent = args.silent;

    // TODO error handling for file rw

    let input_stream: Box<dyn Read> = match args.file {
        Some(file) => Box::new(File::open(file).unwrap()),
        None => Box::new(io::stdin()),
    };

    let mut output_stream: Box<dyn Write> = match args.output {
        Some(file) => Box::new(File::create(file).unwrap()),
        None => Box::new(io::stdout()),
    };

    log!(silent, "Reading...");
    let data = io::read_to_string(input_stream)?;

    // TODO error messages
    log!(silent, "Parsing...");
    let res = parse_all(&data)?;

    log!(silent, "Serializing...");
    let res = serialize(&res)?;

    log!(silent, "Writing...");
    output_stream.write(&res)?;

    log!(silent, "Done!");
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
