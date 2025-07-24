#![cfg(feature = "encoding")]
use core::str::FromStr;

use crate::makura_alloc::{String, ToString, Vec};

use super::Base;

mod base16;
mod base32;
mod base45;
mod base64;

use base16::base16_encode;
use base32::base32_encode;
use base32::base32_hex_encode;
use base45::base45_encode;
use base64::base64_encode;
use base64::base64_url_encode;

/// exposes feature enabled base encodings
pub trait Encode {
    // converts self into &[u8]
    fn to_bytes(&self) -> Vec<u8>;

    ///
    /// ```
    /// let s = "encode me senpai";
    /// let e = s.encode::<String>();
    /// println!("{}", e);
    /// ```
    ///
    fn encode(&self, base: Base) -> Vec<u8> {
        let input = self.to_bytes();
        match base {
            Base::_64 => base64_encode(input),
            Base::_64URL => base64_url_encode(input),
            Base::_45 => base45_encode(input),
            Base::_32 => base32_encode(input),
            Base::_32HEX => base32_hex_encode(input),
            Base::_16 => base16_encode(input),
        }
    }
}

#[cfg(feature = "serde")]
impl<T> Encode for T
where
    T: serde::Serialize,
{
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_string(&self).unwrap().into_bytes()
    }
}

#[cfg(not(feature = "serde"))]
impl<T> Encode for T
where
    T: core::fmt::Display,
{
    fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

#[cfg(feature = "extends")]
pub trait EncodeExt: Encode {
    fn encode_repeat(&self, base: Base, repeat: usize) -> Vec<u8> {
        let mut val = self.encode(base);
        (1..repeat).into_iter().for_each(|_| {
            val = val.encode(base);
        });

        val
    }

    fn encode_chain(&self, bases: &[Base]) -> Vec<u8> {
        if bases.is_empty() {
            return Vec::new();
        }

        let mut bases = bases.into_iter();
        let mut value = self.encode(*bases.next().unwrap());
        bases.for_each(|b| {
            value = value.encode(*b);
        });

        value.into()
    }
}
