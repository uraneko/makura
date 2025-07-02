#![cfg(any(feature = "base64", feature = "base64_url"))]
use crate::makura_alloc::Vec;
use crate::{BASE64, BASE64URL};

use super::{
    DecodeError,
    chars_range::{LWC, NUM, PAD, UPC},
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
fn into_24bits_bytes(value: Vec<u8>) -> Vec<u32> {
    // NOTE len must be an integra multiple of 4
    value
        .chunks(4)
        // .inspect(|c| println!("{:?}", c))
        .map(|b| {
            let mut mask = 0u32;
            mask |= b[0] as u32;
            mask <<= 6;
            mask |= b[1] as u32;
            mask <<= 6;
            mask |= b[2] as u32;
            mask <<= 6;
            mask |= b[3] as u32;

            mask
        })
        .collect()
}

// get back 8 bit bytes from the 24bits bytes
fn into_8bits_bytes(value: Vec<u32>) -> Vec<u8> {
    let mut bytes = value
        .into_iter()
        .flat_map(|b| {
            [
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

#[cfg(feature = "base64")]
pub fn base64_decode(indices: Vec<u8>) -> Vec<u8> {
    let bytes = into_24bits_bytes(indices);

    into_8bits_bytes(bytes)
}

#[cfg(feature = "base64_url")]
pub fn base64_url_decode(indices: Vec<u8>) -> Vec<u8> {
    let bytes = into_24bits_bytes(indices);

    into_8bits_bytes(bytes)
}

pub fn is_valid_64_len(len: usize) -> Result<(), DecodeError> {
    if len % 4 == 0 {
        Ok(())
    } else {
        Err(DecodeError::InvalidLen { len, base: BASE64 })
    }
}

// NOTE pad = 0 and pad = invalid value are both to be handled by the
// only function calling these fns
//
// this is fine since these are internal fns, not part of the public api
// otherwise, checking irrelevant (0, invalid) values at every is_valid_x_padding fn is a pain
//
// this fn expects pads to be a valid base64 padding value
pub fn is_valid_64_padding(last_byte: u8, pads: u8) -> Result<(), DecodeError> {
    let char = last_byte as char;
    let last_byte = idx_from_char(char, &BASE64);
    if last_byte.is_err() {
        return last_byte.map(|_| ());
    }
    let last_byte = last_byte.unwrap();

    match pads {
        1 if last_byte % 4 == 0 => Ok(()),
        2 if last_byte % 16 == 0 => Ok(()),
        1 | 2 => Err(DecodeError::InvalidLastCharForPadding {
            char,
            idx: last_byte,
            pads,
        }),
        _ => unreachable!("both 0 and invalid values were checked before getting here"),
    }
}

// WARN validator fns should take a ref to the input then
// do their checks on that
pub fn is_valid_64url_padding(last_byte: u8, pads: u8) -> Result<(), DecodeError> {
    let char = last_byte as char;
    let last_byte = idx_from_char(char, &BASE64URL);
    if last_byte.is_err() {
        return last_byte.map(|_| ());
    }
    let last_byte = last_byte.unwrap();

    match pads {
        1 if last_byte % 4 == 0 => Ok(()),
        2 if last_byte % 16 == 0 => Ok(()),
        1 | 2 => Err(DecodeError::InvalidLastCharForPadding {
            char,
            idx: last_byte,
            pads,
        }),
        _ => unreachable!("both 0 and invalid values were checked before getting here"),
    }
}

pub fn chars_are_64(value: &[u8]) -> Result<(), DecodeError> {
    if let Some(e) = value
        .into_iter()
        .map(|c| {
            if UPC.contains(c)
                || LWC.contains(c)
                || NUM.contains(c)
                || [b'+', b'/'].contains(c)
                || *c == PAD
            {
                Ok(())
            } else {
                Err(DecodeError::InvalidChar {
                    char: *c as char,
                    base: BASE64,
                })
            }
        })
        .find(|res| res.is_err())
    {
        return e;
    }

    Ok(())
}

pub fn chars_are_64url(value: &[u8]) -> Result<(), DecodeError> {
    if let Some(e) = value
        .into_iter()
        .map(|c| {
            if UPC.contains(c)
                || LWC.contains(c)
                || NUM.contains(c)
                || [b'-', b'_'].contains(c)
                || *c == PAD
            {
                Ok(())
            } else {
                Err(DecodeError::InvalidChar {
                    char: *c as char,
                    base: BASE64URL,
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
    use super::{
        chars_are_64, chars_are_64url, is_valid_64_len, is_valid_64_padding, is_valid_64url_padding,
    };

    #[test]
    fn test0_64url() {
        let output = "pl-";

        assert_eq!(chars_are_64url(output.as_bytes()), Ok(()));
    }

    #[test]
    fn test1_64url() {
        let output = "sqw_";

        assert_eq!(chars_are_64url(output.as_bytes()), Ok(()));
    }

    #[test]
    fn test0_64() {
        let output = "sqw+";

        assert_eq!(chars_are_64(output.as_bytes()), Ok(()));
    }

    #[test]
    fn test1_64() {
        let output = "sqw/";

        assert_eq!(chars_are_64(output.as_bytes()), Ok(()));
    }

    #[test]
    fn test2_64() {
        let output = "12e2e23cSIJOA";

        assert_eq!(chars_are_64(output.as_bytes()), Ok(()));
    }
}
