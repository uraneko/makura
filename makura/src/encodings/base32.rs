extern crate alloc;
use alloc::vec::Vec;

use crate::DecodeResult;
use crate::decoders::base32_decode;
use crate::encoders::base32_encode;
use crate::{Encoding, input_padding};

/// From rfc
/// Special processing is performed if fewer than 40 bits are available
/// at the end of the data being encoded.  A full encoding quantum is
/// always completed at the end of a body.  When fewer than 40 input bits
/// are available in an input group, bits with value zero are added (on
/// the right) to form an integral number of 5-bit groups.  Padding at
/// the end of the data is performed using the "=" character.  Since all
/// base 32 input is an integral number of octets, only the following
/// cases can arise:
///
/// (1) The final quantum of encoding input is an integral multiple of 40
///     bits; here, the final unit of encoded output will be an integral
///     multiple of 8 characters with no "=" padding.
///
/// (2) The final quantum of encoding input is exactly 8 bits; here, the
///     final unit of encoded output will be two characters followed by
///     six "=" padding characters.
///
/// (3) The final quantum of encoding input is exactly 16 bits; here, the
///     final unit of encoded output will be four characters followed by
///     four "=" padding characters.
///
/// (4) The final quantum of encoding input is exactly 24 bits; here, the
///     final unit of encoded output will be five characters followed by
///     three "=" padding characters.
///
/// (5) The final quantum of encoding input is exactly 32 bits; here, the
///     final unit of encoded output will be seven characters followed by
///     one "=" padding character.

pub struct Base32;

impl Encoding for Base32 {
    const TABLE: &'static [char] = &[
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '2', '3', '4', '5', '6', '7',
    ];

    fn as_str() -> &'static str {
        "Base32"
    }

    fn new() -> Self {
        Self
    }

    fn paddable() -> bool {
        true
    }

    fn is_len_valid(len: usize) -> bool {
        len % 8 == 0
    }

    fn is_padding_valid(padding: u8, last: u8) -> bool {
        let last_char = last as char;
        let last = Self::idx_of_char(last_char);
        let Some(last) = last else { return false };

        match padding {
            1 if last % 8 == 0 => true,
            3 if last % 2 == 0 => true,
            4 if last % 16 == 0 => true,
            6 if last % 4 == 0 => true,
            // 1 | 3 | 4 | 6 => Err(DecodeError::InvalidLastCharForPadding {
            //     char,
            //     idx: last,
            //     pads,
            // }),
            1 | 3 | 4 | 6 => false,
            // both 0 and invalid values were checked before getting here
            _ => false,
        }
    }

    fn encode(self, buf: &[u8]) -> Vec<u8> {
        base32_encode(buf)
    }

    fn decode(self, buf: &[u8]) -> Vec<u8> {
        base32_decode(buf)
    }
}
