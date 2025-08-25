extern crate alloc;
use alloc::vec::Vec;

use crate::DecodeResult;
use crate::decoders::base45_decode;
use crate::encoders::base45_encode;
use crate::{Encoding, input_padding};

pub struct Base45;

impl Encoding for Base45 {
    const TABLE: &'static [char] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ' ', '$', '*', '+', '-', '.', '/', ':',
    ];

    fn as_str() -> &'static str {
        "Base45"
    }

    fn new() -> Self {
        Self
    }

    fn is_len_valid(len: usize) -> bool {
        len % 3 != 1
    }

    fn encode(self, buf: &[u8]) -> Vec<u8> {
        base45_encode(buf)
    }

    fn decode(self, buf: &[u8]) -> Vec<u8> {
        base45_decode(buf)
    }
}
