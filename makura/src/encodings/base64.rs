extern crate alloc;
use alloc::vec::Vec;

use crate::DecodeResult;
use crate::decoders::base64_decode;
use crate::encoders::base64_encode;
use crate::{Encoding, input_padding};

pub struct Base64;

impl Encoding for Base64 {
    const TABLE: &'static [char] = &[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    fn as_str() -> &'static str {
        "Base64"
    }

    fn new() -> Self {
        Self
    }

    fn paddable() -> bool {
        true
    }

    fn is_len_valid(len: usize) -> bool {
        len % 4 == 0
    }

    fn is_padding_valid(padding: u8, last: u8) -> bool {
        let last_char = last as char;
        let last = Self::idx_of_char(last_char);
        let Some(last) = last else { return false };

        match padding {
            1 if last % 4 == 0 => true,
            2 if last % 16 == 0 => true,
            // 1 | 2 => Err(DecodeError::InvalidLastCharForPadding {
            //     char,
            //     idx: last,
            //     pads,
            // }),
            1 | 2 => false,
            // both 0 and invalid values were checked before getting here
            _ => false,
        }
    }

    fn encode(self, buf: &[u8]) -> Vec<u8> {
        base64_encode(buf)
    }

    fn decode(self, buf: &[u8]) -> Vec<u8> {
        base64_decode(buf)
    }
}
