use clap::Parser;
use dialogical::Args;

fn main() {
    let args = Args::parse();

    if let Err(e) = dialogical::main(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
