extern crate alloc;
use alloc::vec::Vec;

use crate::DecodeResult;
use crate::decoders::base64_url_decode;
use crate::encoders::base64_url_encode;
use crate::{Encoding, input_padding};

use super::Base64;
pub struct Base64Url;

impl Encoding for Base64Url {
    const TABLE: &'static [char] = &[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '-', '_',
    ];

    fn as_str() -> &'static str {
        "Base64Url"
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
        Base64::is_padding_valid(padding, last)
    }

    fn encode(self, buf: &[u8]) -> Vec<u8> {
        base64_url_encode(buf)
    }

    fn decode(self, buf: &[u8]) -> Vec<u8> {
        base64_url_decode(buf)
    }
}
