#![cfg(feature = "base16")]
extern crate alloc;
use alloc::vec::Vec;

use crate::Encoding;

pub struct Base16;

impl Encoding for Base16 {
    const TABLE: &'static [char] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    fn as_str() -> &'static str {
        "Base16"
    }

    fn new() -> Self {
        Self
    }

    fn paddable() -> bool {
        true
    }

    fn is_len_valid(len: usize) -> bool {
        len % 2 == 0
    }

    fn encode(self, buf: &[u8]) -> Vec<u8> {
        if buf.is_empty() {
            return Vec::new();
        }

        buf.into_iter()
            .flat_map(|b| [(b >> 4) & 15, b & 15])
            .map(|b| Base16::char_of_idx(b).map(|ch| ch as u8).unwrap())
            .collect::<Vec<u8>>()
    }

    fn decode(self, buf: &[u8]) -> Vec<u8> {
        buf.chunks(2)
            .map(|b| {
                let mut mask = 0u8;
                mask |= b[0];
                mask <<= 4;
                mask |= b[1];

                mask
            })
            .collect()
    }
}

#[cfg(test)]
mod test_validators {
    use super::Base16;
    use crate::{Encoding, input_measurements};

    #[test]
    fn test_16() {
        let output = b"6587AF";
        let (_, len, _) = input_measurements(output);

        assert!(Base16::are_chars_valid(output, len));
    }
}
