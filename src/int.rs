//! Provides an arbitrary-length integer abstraction over a bit vector.

use std::mem::size_of;

use bitvec::prelude::*;
use num_traits::{cast, PrimInt};

/// Options to create an [`Integer`].
#[derive(Copy, Clone, Debug, Default)]
pub struct IntegerOptions {
    /// Whether the integer is signed or not.
    signed: bool,
    /// The number of bits the integer should have.
    size: usize,
    /// The number of significant bits in the integer.
    significant_bits: Option<usize>,
    /// Whether the integer should be sign-extended or not when [`size`] is
    /// more than the number of significant bits in the integer.
    sign_extend: bool,
}

impl IntegerOptions {
    /// Creates a [`IntegerOptions`].
    pub fn new(
        signed: bool,
        size: usize,
        significant_bits: Option<usize>,
        sign_extend: bool,
    ) -> Self {
        Self {
            signed,
            size,
            significant_bits,
            sign_extend,
        }
    }
}

/// Represents an arbitrary length integer.
#[derive(Clone, Debug)]
pub struct Integer<T: BitStore> {
    /// The bits of the integer.
    pub(crate) bits: BitVec<<T as BitStore>::Unalias>,
    /// Whether the integer is negative or not.
    negative: bool,
}

impl<T: BitStore> Integer<T> {
    /// Creates an arbitrary length [`Integer`] from a number and provided options.
    ///
    /// Panics if the number of bits in `elem` are less than the provided size.
    pub fn new(mut elem: T, options: IntegerOptions) -> Self {
        assert!(size_of::<T>() * 8 >= options.size);

        let bitslice = elem.view_bits_mut::<Lsb0>();

        let significant_bits = options
            .significant_bits
            .unwrap_or_else(|| bitslice.len() - bitslice.trailing_zeros());

        if options.sign_extend && options.size > significant_bits {
            for mut bit in &mut bitslice[significant_bits..options.size] {
                *bit = true;
            }
        }

        let bits = bitslice[..options.size].to_bitvec();

        Self {
            negative: options.signed && bits.last().map(|f| *f).unwrap_or_default(),
            bits,
        }
    }

    /// Returns a bit vector representation of the two's complement of the integer's bits.
    pub(crate) fn twos_complement(&self) -> BitVec<<T as BitStore>::Unalias> {
        let mut bits = self.bits.clone();
        make_slice_twos_complement(bits.as_mut_bitslice());
        bits
    }

    /// Converts the integer into its two's complement representation.
    pub fn make_twos_complement(&mut self) {
        make_slice_twos_complement(self.bits.as_mut_bitslice());
        self.update_negative_prop();
    }

    /// Updates the negative property of the integer based on the
    /// MSB if it was previously negative.
    fn update_negative_prop(&mut self) {
        self.negative = self.negative && self.bits.last().map(|f| *f).unwrap_or_default();
    }

    /// The number of bits in the integer.
    pub fn bits(&self) -> usize {
        self.bits.len()
    }

    /// Returns whether the integer is negative.
    pub fn is_negative(&self) -> bool {
        self.negative
    }
}

/// Converts the given bitslice into an integer.
pub(crate) fn int_from_slice<T: PrimInt + std::ops::BitOrAssign, U: BitStore>(
    slice: &BitSlice<U>,
) -> T {
    let mut num = T::zero();

    for (idx, bit) in slice.iter().enumerate() {
        num |= cast::<u8, T>(*bit as u8).unwrap() << idx;
    }

    num
}

/// Converts the bitslice into its two's complement.
fn make_slice_twos_complement<T: BitStore>(slice: &mut BitSlice<T>) {
    if let Some(first_one) = slice.first_one() {
        if let Some(bits) = slice.get_mut(first_one + 1..) {
            for mut bit in bits {
                *bit = !*bit;
            }
        }
    }
}
