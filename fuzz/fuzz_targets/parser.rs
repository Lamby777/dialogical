#![no_main]

use libfuzzer_sys::fuzz_target;

use dialogical::parse_all;

fuzz_target!(|data: &str| {
    let _ = parse_all(&data);
});
