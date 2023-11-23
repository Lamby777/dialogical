#![no_main]

use libfuzzer_sys::fuzz_target;

use dialogical::DgParser;

fuzz_target!(|data: &str| {
    let mut parser = DgParser::default();
    let _ = parser.parse_all(&data);
});
