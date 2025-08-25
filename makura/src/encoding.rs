extern crate alloc;
use alloc::vec::Vec;

use crate::DecodeResult;

// base 64, 32... must implement this
pub trait Encoding {
    const TABLE: &'static [char];

    fn new() -> Self;

    fn as_str() -> &'static str;

    // can this encoding add paddings to the data
    fn paddable() -> bool {
        false
    }

    fn idx_of_char(ch: char) -> Option<u8> {
        Self::TABLE
            .iter()
            .position(|cha| *cha == ch)
            .map(|u| u as u8)
    }

    fn char_of_idx(idx: u8) -> Option<char> {
        Self::TABLE.get(idx as usize).map(|ch| *ch)
    }

    fn contains_char(ch: char) -> bool {
        Self::TABLE.contains(&ch)
    }

    fn is_len_valid(len: usize) -> bool;

    // false by default for encodings that are not paddable
    fn is_padding_valid(padding: u8, last: u8) -> bool {
        false
    }

    // len has padding stripped off
    fn are_chars_valid(buf: &[u8], len: usize) -> bool {
        let chars = Self::TABLE;
        buf[..len - 1]
            .into_iter()
            .all(|ch| chars.contains(&(*ch as char)))
    }

    fn encode(self, buf: &[u8]) -> Vec<u8>;

    fn decode(self, buf: &[u8]) -> Vec<u8>;
}

pub fn input_padding(input: &[u8]) -> u8 {
    let len = input.len();
    let mut pads = 0;
    while input[len - pads - 1] == b'=' {
        pads += 1;
    }

    pads as u8
}

pub fn input_measurements(buf: &[u8]) -> (u8, usize, u8) {
    let padding = input_padding(buf);
    let len = buf.len() - padding as usize - 1;
    let Some(last) = buf.last() else {
        unreachable!("already ruled out an empty input");
    };

    (padding, len, *last)
}
