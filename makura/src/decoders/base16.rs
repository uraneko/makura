#![cfg(feature = "base16")]
use crate::BASE16;
use crate::makura_alloc::Vec;

use super::{
    DecodeError,
    chars_range::{HEX, NUM},
};

fn into_8bits_bytes(value: Vec<u8>) -> Vec<u8> {
    value
        .chunks(2)
        .map(|b| {
            let mut mask = 0u8;
            mask |= b[0];
            mask <<= 4;
            mask |= b[1];

            mask
        })
        .collect()
}

pub fn base16_decode(indices: Vec<u8>) -> Vec<u8> {
    into_8bits_bytes(indices)
}

pub fn is_valid_16_len(len: usize) -> Result<(), DecodeError> {
    if len % 2 == 0 {
        Ok(())
    } else {
        Err(DecodeError::InvalidLen { len, base: BASE16 })
    }
}

pub fn chars_are_16(value: &[u8]) -> Result<(), DecodeError> {
    if let Some(e) = value
        .into_iter()
        .map(|c| {
            if NUM.contains(c) || HEX.contains(c) {
                Ok(())
            } else {
                Err(DecodeError::InvalidChar {
                    char: *c as char,
                    base: BASE16,
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
    use super::chars_are_16;

    #[test]
    fn test_16() {
        let output = "6587AF";

        assert_eq!(chars_are_16(output.as_bytes()), Ok(()));
    }
}
