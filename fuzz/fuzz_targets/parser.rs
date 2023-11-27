#![no_main]

use libfuzzer_sys::fuzz_target;

use dialogical::DgParser;

fuzz_target!(|data: &str| {
    let _ = DgParser::new(std::env::current_dir().unwrap()).parse_all(&data);
});
