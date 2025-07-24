#![cfg(feature = "decoding")]
use crate::{Base, Bases, encoding_checks::*, idx_from_char};
use crate::{
    makura_alloc::{BTreeSet, Cow, String, ToString, Vec, vec},
    makura_core::Utf8Error,
    makura_core::ops,
};

pub mod base16;
pub mod base32;
pub mod base45;
pub mod base64;

use crate::base_consts::{BASE16, BASE32, BASE32HEX, BASE45, BASE64, BASE64URL};

pub type DecodeResult<T> = Result<T, DecodeError>;

pub trait Decode {
    fn to_bytes(&self) -> Vec<u8>;

    // DOCS encloding validation
    // * len matches check
    // * all chars match check
    // * the existence and number of padding chars '='
    //
    /// Deduces the string encoding by process of elimination. Takes a base encoded string.
    /// This method modifies self's inner value in place
    ///
    /// for a version that doesn't modify self (clones the inner value),
    /// use deduce_cloned
    ///
    /// # Error
    ///
    /// returns an `Ok(Base)` if no errors were found and a base was guessed safely, or an `Err(DecodeError)` if:
    ///
    /// * a base was deduced but string contains char(s) that don't belong to that base table
    /// * a base couldn't be deduced
    ///
    /// # Accuracy
    ///
    /// This function's deduction is not always correct for some bases,
    /// an example of this is the integrated decoder tests for base32 hex at `tests/base32_hex.rs`,
    /// test4 function panics when using `decode_deduce` instead of `decode` with a passed
    /// Base value
    ///
    /// this method always returns an error if there is more than 1 valid base
    /// it doesnt do estimations or guesses, only definitive answers
    fn deduce(&self) -> DecodeResult<Base> {
        let mut value = self.to_bytes();

        // fuzzing input = "=" panics
        // if value.iter().all(|b| *b == 61) {
        //     return Err(DecodeError::ZeroValidEncodings);
        // }

        if value.is_empty() {
            return Ok(BASE64);
        }

        let (last, len, pads) = input_meta(&mut value.as_slice());

        let mut bases: Bases = Bases::default()
            .bases()
            .into_iter()
            .filter(|b| {
                b.is_valid_len(len).is_ok()
                    && b.is_valid_padding(last, pads).is_ok()
                    && b.are_valid_chars(&value).is_ok()
            })
            .collect::<Vec<Base>>()
            .into();

        if bases.is_empty() {
            return Err(DecodeError::ZeroValidEncodings);
        } else if bases.len() == 1 {
            return bases
                .bases_mut()
                .pop_first()
                .ok_or(unsafe { core::mem::zeroed() });
        }

        Err(DecodeError::TooManyValidEncodings {
            bases: bases.into(),
        })
    }

    /// decodes a given string
    /// takes encoded string and user provided base of the string encoding
    ///
    /// returns a result of the decoded string value or a `DecodeError`
    ///
    /// # Error
    /// returns an Err when the inner decode function returns an error,
    /// which is when the passed encoded string and encoding base do not match
    ///
    /// * use this method when you know your input string's encoding for sure
    /// * otherwise, use decode_deduce method if not sure about the base encoding of the value string
    ///
    /// Note that `decode_deduce`'a deduction is not alawys correct
    // NOTE was force_decode
    // TODO all decode functions need to add assert_encoding
    // if it errors they error without decoding
    //
    // FIXME since the input chars correctness is not validated at first
    // the fn panics before it gets to invalidate some bad input value
    fn decode(&self, base: Base) -> DecodeResult<Vec<u8>> {
        let value = self.to_bytes();

        // fuzzing input = "=" panics
        // TODO remove this
        // just validate the padding
        // if value.iter().all(|b| *b == 61) {
        //     return Err(DecodeError::ZeroValidEncodings);
        // }

        if value.is_empty() {
            return Ok(Vec::new());
        }

        let (last, len, pads) = input_meta(&mut value.as_slice());

        let valid = base.is_valid_len(len);
        if valid.is_err() {
            return valid.map(|_| Default::default());
        }

        let valid = base.is_valid_padding(last, pads);
        if valid.is_err() {
            return valid.map(|_| Default::default());
        }

        let indices = into_table_idx(&value, base);
        let indices = if indices.is_err() {
            return indices.map(|_| Default::default());
        } else {
            indices.unwrap()
        };

        Ok(match base {
            BASE64 => base64_decode(indices),
            BASE64URL => base64_url_decode(indices),
            BASE45 => base45_decode(indices),
            BASE32 => base32_decode(indices),
            BASE32HEX => base32_hex_decode(indices),
            BASE16 => base16_decode(indices),
        })
    }

    fn decode_deduce(&self) -> DecodeResult<Vec<u8>> {
        let base = self.deduce()?;

        self.decode(base)
    }
}

#[cfg(not(feature = "serde"))]
impl<T> Decode for T
where
    T: core::fmt::Display,
{
    fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

#[cfg(feature = "serde")]
impl<T> Decode for T
where
    T: serde::Serialize,
{
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_string(&self).unwrap().into_bytes()
    }
}

#[cfg(feature = "extends")]
pub trait DecodeExt: Decode {
    fn decode_repeat(&self, base: Base, repeat: usize) -> DecodeResult<Vec<u8>> {
        let mut val = self.to_bytes();
        for _ in 1..repeat {
            val = val.decode(base)?;
        }

        Ok(val)
    }

    fn decode_chain(&self, chain: &[Base]) -> DecodeResult<Vec<u8>> {
        let mut val = self.to_bytes();
        for base in chain {
            val = val.decode(*base)?;
        }

        Ok(val)
    }

    fn decode_repeat_deduce(&self, repeat: usize) -> DecodeResult<Vec<u8>> {
        let base = self.deduce()?;

        self.decode_repeat(base, repeat)
    }

    fn decode_chain_deduce(&self, mut len: usize) -> DecodeResult<Vec<u8>> {
        let mut val = self.decode_deduce()?;

        while len > 1 {
            val = val.decode_deduce()?;
            len -= 1;
        }

        Ok(val)
    }
}







/// errors that can occur during the decoding process of some base encoded input value
#[derive(Debug, PartialEq, Clone)]
pub enum DecodeError {
    /// when decoding an encoded string that is supposed to be of base 16 or 45
    /// both of which can not contain padding '=' chars
    /// yet a padding char was found at the end of the encoded string
    NonPaddableEncoding(Base),
    /// results from trying togenerate a string from a Vec<u8> decoded bytes of an
    /// originally encoded string value
    ///
    /// this variant simply passes on the error value from the alloc::string::String::from_utf8
    /// String method
    Utf8Error(Utf8Error),
    /// string encoding is not any of the implemented base encodings
    /// i.e., it is not base 64, 64url, 45, 32, 32hex or 16 encoded
    ZeroValidEncodings,
    /// deducer has run all checks
    /// but more than one base encoding is valid
    TooManyValidEncodings { bases: Vec<Base> },
    /// occurs only on base64 and 32 encoding variants
    /// and only when there is padding on the encoded value
    /// indicates that the last char which should belong to a subset
    /// of the encoding table was out of that subset
    /// read DOCS section on src/decoders.rs mod deducer_pads
    InvalidLastCharForPadding { char: char, idx: u8, pads: u8 },
    /// padding value is invalid for passed encoding
    InvalidPadding { pads: u8, base: Base },
    /// encoded value chars and passed encoding base dont match
    // NOTE is this not still redundant with UnrecognizedCharForBase variant???
    InvalidChar { char: char, base: Base },
    /// encded value's len doesn't match with passed base
    InvalidLen { len: usize, base: Base },
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // TODO
        // fix clippy error
        // infinite recursion
        write!(f, "{:?}", self)
    }
}

impl core::error::Error for DecodeError {}

// takes input value bytes
//
// returns last byte, len with pads, padding length
// TODO dont remove padding
// just change it to 0
// DOCS fuzzing panic when input = "="
// -> substact with overflow
//
// FIXME fuzzing is revealing too many panics
// related to the padding char '='
// best to just strictly validate the padding presence in decode input
pub fn input_meta(value: &mut &[u8]) -> (u8, usize, u8) {
    let len = value.len();
    let mut pads = 0u8;
    // better just validate that pads < 6
    while value[len - pads as usize - 1] == b'=' {
        pads += 1;
    }
    let last = value[len - pads as usize - 1];

    (last, len, pads)
}

// turns back chars from the encoding table to their table index values
pub fn into_table_idx(value: &[u8], base: Base) -> Result<Vec<u8>, DecodeError> {
    // TODO convert paddings into necessary 0 bytes
    // in case they are not there
    let mut err: Option<DecodeError> = None;
    let val = value
        .into_iter()
        .map(|c| match *c as char {
            '=' => {
                if base == BASE16 || base == BASE45 {
                    // this error is no longer reachable
                    Err(DecodeError::NonPaddableEncoding(base))
                } else {
                    Ok(0)
                }
            }
            val => idx_from_char(val, base),
        })
        .take_while(|res| {
            if let Err(e) = res {
                err = Some(e.clone());

                false
            } else {
                true
            }
        })
        .map(|res| res.unwrap())
        .collect::<Vec<u8>>();

    if let Some(e) = err {
        return Err(e);
    }

    Ok(val)
}

pub mod chars_range {
    use super::*;

    pub const LWC: ops::RangeInclusive<u8> = b'a'..=b'z';
    pub const UPC: ops::RangeInclusive<u8> = b'A'..=b'Z';
    pub const NUM: ops::RangeInclusive<u8> = b'0'..=b'9';
    pub const HEX: ops::RangeInclusive<u8> = b'A'..=b'F';
    pub const N32: ops::RangeInclusive<u8> = b'2'..=b'7';
    pub const PAD: u8 = b'=';
}

#[cfg(feature = "nightly")]
// TODO: fix this; use test/bench api
// this module benchmarks different versions of the deduce_encoding Decoder function
mod bench_deduce_encoding {
    extern crate test;
    use crate::Encode;
    use crate::makura_alloc::Vec;
    use crate::{BASE16, BASE32, BASE32HEX, BASE45, BASE64, BASE64URL};

    use test::Bencher;

    const DATA: &str = "io8yyioljb";

    // NOTE
    // new deduce function
    // make function more robust
    // increased performance
    // fixed a bug where encoding cant be deduced for 32 hex encoding
    // but instead of deducing correctly (32hex) it now deduces to 32
    // this can't be helped as there are no chars from the extended hex table
    // that can allow for the deduction of the base as 32hex and not 32
    // for now, in such cases, use force_decode
    #[bench]
    fn bench_deduce_012(b: &mut Bencher) {
        let encs = [
            DATA.encode(BASE64),
            DATA.encode(BASE64URL),
            DATA.encode(BASE45),
            DATA.encode(BASE32),
            DATA.encode(BASE32HEX),
            DATA.encode(BASE16),
        ];

        b.iter(|| {
            encs.iter().for_each(|e| {
                super::Bases::deduce_default(&e).unwrap();
            })
        });
    }
}

/// this module tests that the decoding errors happen as intended when they are supposed to
#[cfg(test)]
mod test_errors {
    use super::into_table_idx;
    use super::vec;
    use super::{BASE16, BASE32, BASE32HEX, BASE64, BASE64URL};
    use super::{DecodeError, Decode};

    #[test]
    fn zero_valid_encodings() {
        let input = "@";
        let Err(e) = super::Bases::default().deduce_encoding(input) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };

        assert_eq!(e, DecodeError::ZeroValidEncodings);
    }

    #[test]
    // BUG this panicked cause of decoders/base32.rs:37:21:
    // index out of bounds: the len is 4 but the index is 4
    // TODO account for zeroes when less than 1 chunk exists in decoded input
    fn too_many_valid_encodings() {
        let output = "AA==";
        let Err(e) = super::Bases::default().deduce_encoding(output) else {
            unreachable!("this should have been an error");
        };

        assert_eq!(
            e,
            DecodeError::TooManyValidEncodings {
                bases: vec![BASE64, BASE64URL]
            }
        );
    }

    // NOTE this error variant can't be reachaed in current impl
    // since it is superseeded by InvalidPadding variant
    #[test]
    fn non_paddable_encoding() {
        let input = "09==";
        let Err(e) = into_table_idx(input.as_bytes(), BASE16) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };

        assert_eq!(e, DecodeError::NonPaddableEncoding(BASE16));
    }

    #[test]
    fn invalid_len() {
        let input = "123";
        let Err(e) = input.decode(BASE64) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };

        assert_eq!(
            e,
            DecodeError::InvalidLen {
                len: 3,
                base: BASE64
            }
        );
    }

    #[test]
    fn invalid_padding() {
        let output = "AAA=====";
        let Err(e) = output.decode( BASE32) else {
            unreachable!("this should have been an error");
        };

        assert_eq!(
            e,
            DecodeError::InvalidPadding {
                base: BASE32,
                pads: 5
            }
        );
    }

    #[test]
    fn invalid_char() {
        // let input = "VT09PQ==";
        let input = "VT";
        let Err(e) = input.decode( BASE16) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };

        assert_eq!(
            e,
            DecodeError::InvalidChar {
                char: 'V',
                base: BASE16
            }
        );
    }

    #[test]
    fn invalid_last_char_for_padding() {
        let output = "AAAD====";
        let Err(e) = output.decode( BASE32) else {
            unreachable!("this should have been an error");
        };

        assert_eq!(
            e,
            DecodeError::InvalidLastCharForPadding {
                char: 'D',
                pads: 4,
                idx: 3
            }
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn utf8_error() {
        let input = [65,66];

        let Err(DecodeError::Utf8Error(e)) = input.decode(BASE16) else {
            unreachable!("input string is not proper base64 encoded, so how did it pass")
        };
    }
}
