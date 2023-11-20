//!
//! literally rewriting it in rust because i got tired
//! of trying to make modules work properly instead of
//! being able to write actual code
//!
//!  - &Cherry, 11/20/2023
//!

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]

/// P/E/T/S Dialogue Compiler
struct Args {
    #[arg(short, long)]
    output: String,

    // if `None`, read stdin
    file: Option<String>,
}

fn main() {
    // parse args
    let _args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name)
    // }
}
