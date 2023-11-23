//!
//! literally rewriting it in rust because i got tired
//! of trying to make modules work properly instead of
//! being able to write actual code
//!
//!  - &Cherry, 11/20/2023
//!

#![allow(dead_code)] // TODO remove this once we're done
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

macro_rules! log {
    ($silent:expr, $($arg:tt)*) => {
        if !$silent {
            eprintln!($($arg)*);
        }
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse args
    let args = Args::parse();
    let silent = args.silent;

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
    let mut parser = DgParser::default();

    // TODO error messages
    log!(silent, "Parsing...");
    let res = parser.parse_all(&data)?;

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
struct Args {
    /// The output file, or stdout if not specified
    #[arg(short, long)]
    output: Option<String>,

    /// The input file, or stdin if not specified
    file: Option<String>,

    /// Silences progress "info" stderr messages.
    #[arg(short, long)]
    silent: bool,
}
