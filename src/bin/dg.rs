use clap::Parser;
use dialogical::Args;

fn main() {
    let args = Args::parse();

    if let Err(e) = dialogical::cli_main(args, None) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
