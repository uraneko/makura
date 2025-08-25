extern crate alloc;
use alloc::vec::Vec;

use crate::DecodeResult;
use crate::decoders::base32_hex_decode;
use crate::encoders::base32_hex_encode;
use crate::{Encoding, input_padding};

use super::Base32;
pub struct Base32Hex;

impl Encoding for Base32Hex {
    const TABLE: &'static [char] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V',
    ];

    fn as_str() -> &'static str {
        "Base32Hex"
    }

    fn new() -> Self {
        Self
    }

    fn paddable() -> bool {
        true
    }

    fn is_len_valid(len: usize) -> bool {
        len % 8 == 0
    }

    fn is_padding_valid(padding: u8, last: u8) -> bool {
        Base32::is_padding_valid(padding, last)
    }

    fn encode(self, buf: &[u8]) -> Vec<u8> {
        base32_hex_encode(buf)
    }

    fn decode(self, buf: &[u8]) -> Vec<u8> {
        base32_hex_decode(buf)
    }
}
