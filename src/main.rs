//!
//! literally rewriting it in rust because i got tired
//! of trying to make modules work properly instead of
//! being able to write actual code
//!
//!  - &Cherry, 11/20/2023
//!

#![allow(dead_code)]
#![feature(if_let_guard)]

use clap::Parser;
mod comptime;
mod pages;
mod parser;

#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
#[command(author, version, about)]
/// P/E/T/S Dialogue Compiler
struct Args {
    /// The output file, or stdout if not specified
    #[arg(short, long)]
    output: String,

    /// Whether or not to silence script output
    #[arg(short, long)]
    silent: bool,

    /// The input file, or stdin if not specified
    file: Option<String>,
}

fn main() {
    // parse args
    let _args = Args::parse();
}
