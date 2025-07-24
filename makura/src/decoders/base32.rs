#![cfg(any(feature = "base32", feature = "base32_hex"))]
use crate::makura_alloc::Vec;
use crate::{BASE32, BASE32HEX};

use super::{
    DecodeError,
    chars_range::{N32, NUM, PAD, UPC},
    idx_from_char,
};

/// DOCS
/// last 3 octets
/// (1) The final quantum of encoding input is an integral multiple of 24
///     bits; here, the final unit of encoded output will be an integral
///     multiple of 4 characters with no "=" padding.
///
/// (2) The final quantum of encoding input is exactly 8 bits; here, the
///     final unit of encoded output will be two characters followed by
///     two "=" padding characters.
///
/// (3) The final quantum of encoding input is exactly 16 bits; here, the
///     final unit of encoded output will be three characters followed by
///     one "=" padding character.

// to implement the other decoders
// only a different version of this function is needed
// the other functions stay the same
fn into_40bits_bytes(value: Vec<u8>) -> Vec<u64> {
    // NOTE len must be an integra multiple of 4
    value
        .chunks(8)
        .map(|b| {
            let mut mask = 0u64;
            mask |= b[0] as u64;
            mask <<= 5;
            mask |= b[1] as u64;
            mask <<= 5;
            mask |= b[2] as u64;
            mask <<= 5;
            mask |= b[3] as u64;
            mask <<= 5;
            mask |= b[4] as u64;
            mask <<= 5;
            mask |= b[5] as u64;
            mask <<= 5;
            mask |= b[6] as u64;
            mask <<= 5;
            mask |= b[7] as u64;

            mask
        })
        .collect()
}

// get back 8 bit bytes from the 24bits bytes
fn into_8bits_bytes(value: Vec<u64>) -> Vec<u8> {
    let mut bytes = value
        .into_iter()
        .flat_map(|b| {
            [
                // same as ( b >> 32 ) as u8
                ((b & 0xff00000000) >> 32) as u8,
                ((b & 0xff000000) >> 24) as u8,
                ((b & 0xff0000) >> 16) as u8,
                ((b & 0xff00) >> 8) as u8,
                b as u8,
            ]
        })
        .collect::<Vec<u8>>();
    while let Some(0) = bytes.last() {
        bytes.pop();
    }

    bytes
}

#[cfg(feature = "base32")]
pub fn base32_decode(indices: Vec<u8>) -> Vec<u8> {
    let bytes = into_40bits_bytes(indices);

    into_8bits_bytes(bytes)
}

#[cfg(feature = "base32_hex")]
pub fn base32_hex_decode(indices: Vec<u8>) -> Vec<u8> {
    let bytes = into_40bits_bytes(indices);

    into_8bits_bytes(bytes)
}

pub fn is_valid_32_len(len: usize) -> Result<(), DecodeError> {
    if len % 8 == 0 {
        Ok(())
    } else {
        Err(DecodeError::InvalidLen { len, base: BASE32 })
    }
}

pub fn is_valid_32hex_padding(last_byte: u8, pads: u8) -> Result<(), DecodeError> {
    let char = last_byte as char;
    let last_byte = idx_from_char(char, BASE32HEX);
    if last_byte.is_err() {
        return last_byte.map(|_| ());
    }
    let last_byte = last_byte.unwrap();

    match pads {
        1 if last_byte % 8 == 0 => Ok(()),
        3 if last_byte % 2 == 0 => Ok(()),
        4 if last_byte % 16 == 0 => Ok(()),
        6 if last_byte % 4 == 0 => Ok(()),
        1 | 3 | 4 | 6 => Err(DecodeError::InvalidLastCharForPadding {
            char,
            idx: last_byte,
            pads,
        }),
        _ => unreachable!("both 0 and invalid values were checked before getting here"),
    }
}

pub fn is_valid_32_padding(last_byte: u8, pads: u8) -> Result<(), DecodeError> {
    let char = last_byte as char;
    let last_byte = idx_from_char(char, BASE32);
    if last_byte.is_err() {
        return last_byte.map(|_| ());
    }
    let last_byte = last_byte.unwrap();

    match pads {
        1 if last_byte % 8 == 0 => Ok(()),
        3 if last_byte % 2 == 0 => Ok(()),
        4 if last_byte % 16 == 0 => Ok(()),
        6 if last_byte % 4 == 0 => Ok(()),
        1 | 3 | 4 | 6 => Err(DecodeError::InvalidLastCharForPadding {
            char,
            idx: last_byte,
            pads,
        }),
        _ => unreachable!("both 0 and invalid values were checked before getting here"),
    }
}

pub fn chars_are_32(value: &[u8]) -> Result<(), DecodeError> {
    if let Some(e) = value
        .into_iter()
        .map(|c| {
            if UPC.contains(c) || N32.contains(c) || *c == PAD {
                Ok(())
            } else {
                Err(DecodeError::InvalidChar {
                    char: *c as char,
                    base: BASE32,
                })
            }
        })
        .find(|e| e.is_err())
    {
        return e;
    }

    Ok(())
}

pub fn chars_are_32hex(value: &[u8]) -> Result<(), DecodeError> {
    if let Some(e) = value
        .into_iter()
        .map(|c| {
            if NUM.contains(c) || (b'A'..=b'V').contains(c) || *c == PAD {
                Ok(())
            } else {
                Err(DecodeError::InvalidChar {
                    char: *c as char,
                    base: BASE32HEX,
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
    use super::{chars_are_32, chars_are_32hex};

    #[test]
    fn test_32hex() {
        let output = "49312ASC";

        assert_eq!(chars_are_32hex(output.as_bytes()), Ok(()));
    }

    #[test]
    #[should_panic]
    fn fail_32hex() {
        let output = "697JHGX";

        assert_eq!(chars_are_32hex(output.as_bytes()), Ok(()));
    }

    #[test]
    fn test_32() {
        let output = "AZSX5672";

        assert_eq!(chars_are_32(output.as_bytes()), Ok(()));
    }

    #[test]
    #[should_panic]
    fn fail_32() {
        let output = "1SA";

        assert_eq!(chars_are_32(output.as_bytes()), Ok(()));
    }
}
