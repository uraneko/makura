use crate::makura_alloc::{BTreeSet, Cow, String, Vec, vec};
use crate::{Base, DecodeError, DecodeResult, base_consts::*, decoders::input_meta};

/// a set of bases (Base)
///
/// uses a BTreeSet for its inner value
#[derive(Debug, Clone)]
pub struct Bases {
    bases: BTreeSet<Base>,
}

impl Default for Bases {
    fn default() -> Self {
        Self {
            bases: BTreeSet::from_iter([BASE32, BASE32HEX, BASE16, BASE45, BASE64, BASE64URL]),
        }
    }
}

impl From<Vec<Base>> for Bases {
    fn from(value: Vec<Base>) -> Self {
        Self {
            bases: value.into_iter().collect(),
        }
    }
}

impl From<Bases> for Vec<Base> {
    fn from(value: Bases) -> Self {
        value.bases.into_iter().collect()
    }
}

impl From<&[Base]> for Bases {
    fn from(value: &[Base]) -> Self {
        Self {
            bases: value.into_iter().map(|b| *b).collect(),
        }
    }
}

impl From<&mut Bases> for Vec<Base> {
    fn from(value: &mut Bases) -> Self {
        let mut val = core::mem::take(value);

        val.bases().into_iter().collect()
    }
}

impl Bases {
    pub const DEFAULT: [Base; 6] = [BASE64, BASE64URL, BASE45, BASE32, BASE32HEX, BASE16];

    /// returns a new Bases with an empty BTreeSet
    pub fn new() -> Self {
        Self {
            bases: BTreeSet::new(),
        }
    }

    /// delegation of BTreeSet's contains method
    pub fn contains(&self, base: Base) -> bool {
        self.bases.contains(&base)
    }

    /// delegation of BTreeSet's insert method
    pub fn insert(&mut self, base: Base) -> bool {
        self.bases.insert(base)
    }

    /// delegation of BTreeSet's remove method
    pub fn remove(&mut self, base: Base) -> bool {
        self.bases.remove(&base)
    }

    /// delegation of BTreeSet's clear method
    pub fn clear(&mut self) {
        self.bases.clear()
    }

    /// delegation of BTreeSet's is_empty method
    pub fn is_empty(&self) -> bool {
        self.bases.is_empty()
    }

    /// delegation of BTreeSet's len method
    pub fn len(&self) -> usize {
        self.bases.len()
    }

    /// returns the owned inner value,
    ///  doesnt consume self
    ///
    /// changes self's inner value to BTreeSet::default() | new()
    pub fn bases(&mut self) -> BTreeSet<Base> {
        core::mem::take(&mut self.bases)
    }

    /// returns an immutable reference to the inner BTreeSet value
    pub fn bases_ref(&self) -> &BTreeSet<Base> {
        &self.bases
    }

    /// returns a mutable reference to the inner BTreeSet value
    pub fn bases_mut(&mut self) -> &mut BTreeSet<Base> {
        &mut self.bases
    }

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
    pub fn deduce_encoding<T: AsRef<[u8]>>(&mut self, value: T) -> Result<Base, DecodeError> {
        let mut value = value.as_ref();

        // fuzzing input = "=" panics
        // if value.iter().all(|b| *b == 61) {
        //     return Err(DecodeError::ZeroValidEncodings);
        // }

        if value.is_empty() {
            return Ok(BASE64);
        }

        let (last, len, pads) = input_meta(&mut value);

        *self = Self {
            bases: self
                .bases()
                .into_iter()
                .filter(|b| {
                    b.is_valid_len(len).is_ok()
                        && b.is_valid_padding(last, pads).is_ok()
                        && b.are_valid_chars(value).is_ok()
                })
                .collect(),
        };

        if self.is_empty() {
            return Err(DecodeError::ZeroValidEncodings);
        } else if self.len() == 1 {
            return self
                .bases_mut()
                .pop_first()
                .ok_or(unsafe { core::mem::zeroed() });
        }

        Err(DecodeError::TooManyValidEncodings { bases: self.into() })
    }

    /// same as deduce_encoding but this method will not error out
    /// when it finds more than 1 valid encoding
    ///
    /// instead, it will take the first encoding of self.bases as the correct encoding
    ///
    /// basically this considers the passed bases to be sorted
    /// and the least values (bases[0], base[1]...) as the most likely correct answer
    pub fn deduce_sorted<T: AsRef<[u8]>>(&mut self, value: T) -> Result<Base, DecodeError> {
        let mut value = value.as_ref();

        // fuzzing input = "=" panics
        // if value.iter().all(|b| *b == 61) {
        //     return Err(DecodeError::ZeroValidEncodings);
        // }

        if value.is_empty() {
            return Ok(BASE64);
        }

        let (last, len, pads) = input_meta(&mut value);

        *self = Self {
            bases: self
                .bases()
                .into_iter()
                .filter(|b| {
                    b.is_valid_len(len).is_ok()
                        && b.is_valid_padding(last, pads).is_ok()
                        && b.are_valid_chars(value).is_ok()
                })
                .collect(),
        };

        if self.is_empty() {
            return Err(DecodeError::ZeroValidEncodings);
        } else if self.len() == 1 {
            return self
                .bases_mut()
                .pop_first()
                .ok_or(unsafe { core::mem::zeroed() });
        } else if self.len() == 2 && self.contains(BASE64) && self.contains(BASE64URL) {
            // WARN it is pretty common to have both base64 and 64url remain together
            // since '/' is very rare and '+' is a bit less rarer
            // so we prioritize base64
            return Ok(BASE64);
        }

        Ok(self.bases.pop_first().unwrap())
    }

    /// calls self's deduce_encoding on Self::default,
    /// which is all 6 known bases
    /// takes the input value to be analyzed
    pub fn deduce_default<T: AsRef<[u8]>>(value: T) -> Result<Base, DecodeError> {
        Self::default().deduce_sorted(value)
    }
}
