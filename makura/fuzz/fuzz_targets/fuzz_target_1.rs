#![no_main]

use libfuzzer_sys::fuzz_target;
use makura::Decoder;

fuzz_target!(|data: &[u8]| {
    if let Ok(i) = str::from_utf8(data) {
        println!("||||||||||||||||||||||{:?}", Decoder::decode_deduce(i));
    }
});
