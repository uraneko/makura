use crate::{DecodeResult, DecodeError};
use crate::encoding_checks::*;

#[derive(PartialEq, Default, Clone, Copy, Ord, PartialOrd, Eq, Hash)]
// #[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum Base {
    #[default]
    _64,
    _64URL,
    _45,
    _32,
    _32HEX,
    _16,
}

pub const BASE64: Base = Base::_64;
pub const BASE64URL: Base = Base::_64URL;
pub const BASE32: Base = Base::_32;
pub const BASE32HEX: Base = Base::_32HEX;
pub const BASE16: Base = Base::_16;
pub const BASE45: Base = Base::_45;

impl core::fmt::Debug for Base {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::_64 => "Base64",
                Self::_64URL => "Base64URL",
                Self::_45 => "Base45",
                Self::_32 => "Base32",
                Self::_32HEX => "Base32HEX",
                Self::_16 => "Base16",
            }
        )
    }
}

impl core::fmt::Display for Base {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::_64 => "Base64",
                Self::_64URL => "Base64URL",
                Self::_45 => "Base45",
                Self::_32 => "Base32",
                Self::_32HEX => "Base32HEX",
                Self::_16 => "Base16",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BaseError {
    InvalidStrBaseValue,
}

impl core::fmt::Display for BaseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidStrBaseValue =>
                    "Received invalid string value for base name, string base name should be one of [64, 64URL, 45, 32, 32HEX, 16]",
            }
        )
    }
}

impl core::error::Error for BaseError {}

impl core::str::FromStr for Base {
    type Err = BaseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "64" => Ok(BASE64),
            "64url" => Ok(BASE64URL),
            "45" => Ok(BASE45),
            "32" => Ok(BASE32),
            "32hex" => Ok(BASE32HEX),
            "16" => Ok(BASE16),
            _ => Err(BaseError::InvalidStrBaseValue),
        }
    }
}

impl Base {
    // first 26 values of base encoding table are the uppercase alphabet letters A -> Z
    pub fn alpha_26(&self) -> bool {
        self == &Self::_64 || self == &Self::_64URL || self == &Self::_32
    }

    // first 16 values in the base encoding table are the base 16 numbers 0 -> F
    pub fn hex_16(&self) -> bool {
        self == &Self::_32HEX || self == &Self::_16
    }

    // base is 64 or 64 url
    pub fn is_any_64(&self) -> bool {
        self == &Self::_64 || self == &Self::_64URL
    }

    // base is strictly 32
    pub fn is_32(&self) -> bool {
        self == &Self::_32
    }

    // base is strictly 32 hex
    pub fn is_32_hex(&self) -> bool {
        self == &Self::_32HEX
    }

    pub fn is_45(&self) -> bool {
        self == &Self::_45
    }
}

impl Base {
    // DOCS:
    // technically we can not get a B, C or D at the end of a byte
    // we can only get such values at the beginning of a byte
    // let me elaborate
    // for an input value = 0b0000_0001
    // the output value will be = 0b00000, 0b001
    // the second bit will then be padded by 2 negative bits 00
    // rendering an output of: 0b00000, 0b00100 -> AE
    // so to say,the smallest positive bit value of 1 can never be generated at the end of a byte
    // this is the case for 1,2 and 3 they can only be at the start of a byte like so: 0b0000_100,
    // taking the first 5 bits; the first encoded value will be a B
    // consequently, we can never get any values in between 0 and 4 in a base32 encoding from the first
    // u5 byte,
    // that is, if we have a 2 chars input value starting with some_char
    // the second char can only be
    // the 0th, 4th, 8th, 12th, 16th... char in the base32 encoding table
    // this is because we always pad the second value by 2 zeroes
    // and we do that, the smallest value of the second u5 byte is 0 followed by 100 which is four
    // all possible values of the second byte will have to be multiples of 4
    //
    // in conclusion: for every input value I which is base32 encoded, assuming that I is padded
    // such that NP is the number of padding chars and CL is the length of the chunk containing the last bytes:
    // -> NP depends upon CL, e.g., if CL = 1
    // => 1 byte of 1st 5 bits and 2nd byte of last 3 bits (padded by 00) = 2 bytes in chunk
    // =>  NP = 8 - 2 = 6
    // there can only be the following cases for the smallest non zero value of the last byte(u5) LB:
    // * if CL = 1 && NP = 6 => LB = 001
    // -> padded by least bits 00 => LB = 00100, is always a multiple of 4
    //
    // * if CL = 2 && NP = 4 => LB = 1
    // -> padded by least bits 0000 => LB = 10000, is always a multiple of 16
    //
    // * if CL = 3 && NP = 3 => LB = 0001
    // -> padded by least bit 0 => LB = 00010, is always a multiple of 2
    //
    // * if CL = 4 && NP = 1 => LB = 01
    // -> padded by least bits 000 => LB = 01000, is always a multiple of 8
    //
    // * if CL = 5 && NP = 0 => the last value can be any value in the base32 encoding table
    //
    //
    // likewise for base64, there can only be the following padded input cases:
    // * if CL = 1 && NP = 2 => LB = 01
    // -> padded by least bits 0000 => LB = 010_000, is always a multiple of 16
    //
    // * if CL = 2 && NP = 1 => LB = 0001;
    // -> padded by least bits 00 => LB = 000_100, is always a multiple of 4
    // ^_ since [16 = 6 * 2 + 4] we already have 3 values, but add
    // a padding char to indicate that the last byte value was padded by least bits 00
    //
    // * if CL = 3 && NP = 0 => the last value can be any value in the base64 encoding table
    pub fn is_valid_padding(&self, last_byte: u8, pads: u8) -> DecodeResult<()>  {
        if pads == 0 {
            return Ok(());
        }

        match pads {
            1 if BASE64 == *self => is_valid_64_padding(last_byte, pads),
            1 if BASE64URL == *self => is_valid_64url_padding(last_byte, pads),
            1 if BASE32 == *self => is_valid_32_padding(last_byte, pads),
            1 if BASE32HEX == *self => is_valid_32hex_padding(last_byte, pads),
            2 if BASE64 == *self => is_valid_64_padding(last_byte, pads),
            2 if BASE64URL == *self => is_valid_64url_padding(last_byte, pads),
            1 | 2 => Err(DecodeError::NonPaddableEncoding(*self)),
            3 | 4 | 6 if BASE32 == *self => is_valid_32_padding(last_byte, pads),
            3 | 4 | 6 if BASE32HEX == *self => is_valid_32hex_padding(last_byte, pads),
            _ if BASE45 == *self || BASE16 == *self => Err(DecodeError::NonPaddableEncoding(*self)),
            _ => Err(DecodeError::InvalidPadding { base: *self, pads }),
        }
    }

    // NOTE this doesnt differenciate between hex and url variants
    // the len checks should go first  <- least costly
    // then the pad checks            <- in between
    // then finally the chars checks <- costliest
    pub fn is_valid_len(&self, len: usize) -> Result<(), DecodeError> {
        match *self {
            BASE64 | BASE64URL => is_valid_64_len(len),
            BASE45 => is_valid_45_len(len),
            BASE32 | BASE32HEX => is_valid_32_len(len),
            BASE16 => is_valid_16_len(len),
        }
    }

    /// checks whether all bytes of input
    /// match self's value
    pub fn are_valid_chars(&self, input: &[u8]) -> Result<(), DecodeError> {
        match *self {
            // FIXME it's redundant to do both a 64 and a 64 url checks
            BASE64 => chars_are_64(input),
            BASE64URL => chars_are_64url(input),
            BASE45 => chars_are_45(input),
            BASE32 => chars_are_32(input),
            BASE32HEX => chars_are_32hex(input),
            BASE16 => chars_are_16(input),
        }
    }
}

// trait ValidEncoding<B> where B: EncodingBase {
//     
// }
//
// trait EncodingBase {
//
// }
