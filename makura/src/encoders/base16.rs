#![cfg(feature = "base16")]
use crate::makura_alloc::{String, Vec};

use crate::BASE16;
use crate::char_from_idx;

fn into_4bits_bytes(bytes: Vec<u8>) -> Vec<u8> {
    let bytes = bytes.into_iter();
    // let mut last = bytes.next_back().unwrap();

    bytes.flat_map(|b| [(b >> 4) & 15, b & 15]).collect()
}

fn into_base16(bytes: Vec<u8>) -> Vec<u8> {
    bytes
        .into_iter()
        .map(|b| char_from_idx(b, &BASE16) as u8)
        .collect::<Vec<u8>>()
}

pub fn base16_encode(value: Vec<u8>) -> Vec<u8> {
    if value.is_empty() {
        return Vec::new();
    }

    let bytes = into_4bits_bytes(value);

    into_base16(bytes)
}
