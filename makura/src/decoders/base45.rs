#![cfg(feature = "base45")]
use crate::BASE45;
use crate::makura_alloc::Vec;

use super::{
    DecodeError,
    chars_range::{NUM, UPC},
};

// DOCS fuzzing gave a panic here, inside the `let last = {...} block`
// it was an attempt to multiply with overflow
// could fix by using u32 instead of u16
// recorded panic input cases:
// `2JY`
// `2.Y`
fn into_base45_values(bytes: Vec<u8>) -> Vec<u32> {
    let mut chunks = bytes.chunks(3);
    let last = chunks.next_back().unwrap();

    let mut values: Vec<u32> = chunks
        // .inspect(|c| println!("{:?}", c))
        .map(|b| b[2] as u32 * 45 * 45 + b[1] as u32 * 45 + b[0] as u32)
        .collect();

    let last = {
        if last.len() == 3 {
            last[2] as u32 * 45 * 45 + last[1] as u32 * 45 + last[0] as u32
        } else if last.len() == 2 {
            last[1] as u32 * 45 + last[0] as u32
        } else {
            unreachable!("last chunk len can only be 2 or 3");
        }
    };
    values.push(last);

    values
}

// get back 8 bit bytes from the 24bits bytes
fn into_base265_values(value: Vec<u32>) -> Vec<u8> {
    let mut bytes = value.into_iter();
    let last = bytes.next_back().unwrap();
    let mut bytes = bytes
        .flat_map(|b| [((b & 0xff00) >> 8) as u8, b as u8])
        .collect::<Vec<u8>>();

    if last < u8::MAX as u32 {
        bytes.push(last as u8);
    } else {
        bytes.push((last >> 8) as u8);
        bytes.push(last as u8);
    }

    bytes
}

pub fn base45_decode(indices: Vec<u8>) -> Vec<u8> {
    let bytes = into_base45_values(indices);

    into_base265_values(bytes)
}

pub fn is_valid_45_len(len: usize) -> Result<(), DecodeError> {
    if len % 3 != 1 {
        Ok(())
    } else {
        Err(DecodeError::InvalidLen { len, base: BASE45 })
    }
}

pub fn chars_are_45(value: &[u8]) -> Result<(), DecodeError> {
    if let Some(e) = value
        .into_iter()
        .map(|c| {
            if NUM.contains(c)
                || UPC.contains(c)
                || [b' ', b'$', b'%', b'*', b'+', b'-', b'.', b'/', b':'].contains(c)
            {
                Ok(())
            } else {
                Err(DecodeError::InvalidChar {
                    char: *c as char,
                    base: BASE45,
                })
            }
        })
        .find(|e| e.is_err())
    {
        return e;
    }

    Ok(())
}

#[cfg(test)]
mod test_validators {
    use super::chars_are_45;

    #[test]
    fn test0_45() {
        let output = "CSAL $%*+-./:";

        assert_eq!(chars_are_45(output.as_bytes()), Ok(()));
    }
}
