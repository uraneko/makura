#![cfg(any(feature = "base32", feature = "base32_hex"))]
use crate::makura_alloc::{String, Vec};

use crate::char_from_idx;
use crate::{BASE32, BASE32HEX};

// separates the input string into chunks of 24bits
// bytes_of_u40
fn into_40bits_chunks(data: &[u8]) -> impl core::iter::DoubleEndedIterator<Item = u64> {
    let mut bytes = data.chunks(5);
    // println!("{:?}", bytes.clone().collect::<Vec<&[u8]>>());
    let last = {
        let last = bytes.next_back().unwrap();
        let mut mask = 0u64;
        mask |= last[0] as u64;
        mask <<= 8;
        mask |= if last.len() < 2 { 0u64 } else { last[1] as u64 };
        mask <<= 8;
        mask |= if last.len() < 3 { 0u64 } else { last[2] as u64 };
        mask <<= 8;
        mask |= if last.len() < 4 { 0u64 } else { last[3] as u64 };
        mask <<= 8;
        mask |= if last.len() < 5 { 0u64 } else { last[4] as u64 };

        mask
    };

    bytes
        .map(|b| {
            let mut mask = 0u64;
            mask |= b[0] as u64;
            mask <<= 8;
            mask |= b[1] as u64;
            mask <<= 8;
            mask |= b[2] as u64;
            mask <<= 8;
            mask |= b[3] as u64;
            mask <<= 8;
            mask |= b[4] as u64;

            mask
        })
        .chain(Some(last))
}

// bytes_of_u5
fn into_5bits_bytes(
    bytes: impl core::iter::DoubleEndedIterator<Item = u64>,
) -> impl core::iter::DoubleEndedIterator<Item = u8> {
    let bytes = bytes.into_iter();
    // let mut last = bytes.next_back().unwrap();

    bytes.flat_map(|b| {
        [
            // NOTE & 31 to take only the least 5 bits
            (b >> 35) as u8 & 31,
            (b >> 30) as u8 & 31,
            (b >> 25) as u8 & 31,
            (b >> 20) as u8 & 31,
            (b >> 15) as u8 & 31,
            (b >> 10) as u8 & 31,
            (b >> 5) as u8 & 31,
            b as u8 & 31,
        ]
    })
}

fn into_base32(bytes: impl core::iter::DoubleEndedIterator<Item = u8>) -> Vec<u8> {
    let mut cd = 6;
    let mut pad = true;
    bytes
        .rev()
        .map(|b| {
            if cd > 0 && pad && b == 0 {
                cd -= 1;
                b'='
            } else {
                pad = false;
                char_from_idx(b, &BASE32) as u8
            }
        })
        .rev()
        .collect()
}

fn into_base32_hex(bytes: impl core::iter::DoubleEndedIterator<Item = u8>) -> Vec<u8> {
    let mut cd = 6;
    let mut pad = true;
    bytes
        .rev()
        // .inspect(|b| println!("{}", b))
        .map(|b| {
            if cd > 0 && pad && b == 0 {
                cd -= 1;
                b'='
            } else {
                pad = false;
                char_from_idx(b, &BASE32HEX) as u8
            }
        })
        .rev()
        .collect()
}

#[cfg(feature = "base32")]
pub fn base32_encode(value: &[u8]) -> Vec<u8> {
    let value = value.as_ref();
    if value.is_empty() {
        return "".into();
    }

    let chunks = into_40bits_chunks(value);
    let bytes = into_5bits_bytes(chunks);

    into_base32(bytes)
}

#[cfg(feature = "base32_hex")]
pub fn base32_hex_encode(value: &[u8]) -> Vec<u8> {
    let value = value.as_ref();
    if value.is_empty() {
        return "".into();
    }

    let chunks = into_40bits_chunks(value);
    let bytes = into_5bits_bytes(chunks);

    into_base32_hex(bytes)
}
