#![no_main]

use libfuzzer_sys::fuzz_target;
use makura::Decode;

fuzz_target!(|data: &[u8]| {
    if let Ok(i) = std::str::from_utf8(data) {
        println!("||||||||||||||||||||||{:?}", i.decode_deduce());
    }
});
