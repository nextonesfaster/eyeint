//! Provides formatting utilities for [`Integer`](crate::int::Integer).

use std::fmt::{Binary, Display, LowerHex, Octal, UpperHex};

use bitvec::{slice::BitSlice, store::BitStore};

use crate::int::{int_from_slice, Integer};

impl<T: BitStore> Display for Integer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp_slice;
        let slice = if self.is_negative() {
            temp_slice = self.twos_complement();
            &temp_slice
        } else {
            &self.bits
        };

        if self.is_negative() {
            // (-(num_from_slice::<u64, _>(slice) as i64)).fmt(f)
            Display::fmt(&-(int_from_slice::<u64, _>(slice) as i64), f)
        } else {
            Display::fmt(&int_from_slice::<u64, _>(slice), f)
        }
    }
}

impl<T: BitStore> Binary for Integer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.bits
            .iter()
            .rev()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>()
            .fmt(f)
    }
}

impl<T: BitStore> Octal for Integer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.bits
            .chunks(3)
            .map(|c| bits_to_char(c, 8))
            .rev()
            .collect::<String>()
            .fmt(f)
    }
}

impl<T: BitStore> LowerHex for Integer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.bits
            .chunks(4)
            .map(|c| bits_to_char(c, 16))
            .rev()
            .collect::<String>()
            .fmt(f)
    }
}

impl<T: BitStore> UpperHex for Integer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.bits
            .chunks(4)
            .map(|c| bits_to_char(c, 16).to_ascii_lowercase())
            .rev()
            .collect::<String>()
            .fmt(f)
    }
}

pub trait FormatBits {
    /// Returns a binary representation of the bits.
    fn binary_string(&self) -> String;

    /// Returns an octal representation of the bits.
    fn octal_string(&self) -> String;

    /// Returns a hex representation of the bits.
    fn hex_string(&self) -> String;
}

impl<T: BitStore> FormatBits for Integer<T> {
    fn binary_string(&self) -> String {
        self.bits
            .iter()
            .rev()
            .map(|b| if *b { '1' } else { '0' })
            .collect()
    }

    fn octal_string(&self) -> String {
        self.bits
            .chunks(3)
            .map(|c| bits_to_char(c, 8))
            .rev()
            .collect()
    }

    fn hex_string(&self) -> String {
        self.bits
            .chunks(4)
            .map(|c| bits_to_char(c, 16))
            .rev()
            .collect()
    }
}

/// Converts the bitslice into a character based on the given radix.
///
/// The length of the bitslice should be appropriate for the radix. For instance,
/// the length should be at most 3 for radix 8 (octal), and 4 for radix 16 (hex).
fn bits_to_char<T: BitStore>(slice: &BitSlice<T>, radix: u32) -> char {
    char::from_digit(int_from_slice(slice), radix).expect("bitslice of valid length")
}
