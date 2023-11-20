/*
* def parse_args():
    parser = ArgumentParser()
    parser.add_argument("-o", "--output", help="output file name")

    g = parser.add_mutually_exclusive_group(required=True)
    g.add_argument("-v", "--version", help="version info", action="store_true")
    g.add_argument("-s", "--stdin", help="read from stdin", action="store_true")
    g.add_argument("file", help="definition file", nargs="?")

    # show help if no args given
    # "borrowed, not stolen"
    # - Ferris the Crab
    return parser.parse_args(args=None if sys.argv[1:] else ["--help"])

* */

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    // parse args
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
