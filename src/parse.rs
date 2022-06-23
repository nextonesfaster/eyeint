//! Utilities to parse integers from a string.

use num_traits::PrimInt;

/// Parses the given string into an integer of the specified radix.
///
/// Returns the parsed integer and an optional number of significant bits in the
/// in the input.
///
/// The number of significant bits is only returned when input is a binary, octal,
/// or hexadecimal string.
pub fn parse<I: PrimInt>(str: &str, radix: u32) -> Result<(I, Option<usize>), I::FromStrRadixErr> {
    let parse_with_sig_bits = |prefix: &str| -> Result<(I, Option<usize>), I::FromStrRadixErr> {
        let trimmed = str
            .trim_start_matches(prefix)
            .trim_start_matches(&prefix.to_ascii_uppercase());
        I::from_str_radix(trimmed, radix).map(|i| (i, Some(trimmed.len())))
    };

    match radix {
        2 => parse_with_sig_bits("0b"),
        8 => parse_with_sig_bits("0o").map(|(i, b)| (i, Some(b.unwrap() * 3))),
        16 => parse_with_sig_bits("0x").map(|(i, b)| (i, Some(b.unwrap() * 4))),
        _ => I::from_str_radix(str, radix).map(|i| (i, None)),
    }
}

/// Tries to identify radix of the integer string.
///
/// The first two characters of the string are used to determine the radix.
/// The characters to radix mapping is as follows:
///
/// - `0b` -> 2 (binary)
/// - `0o` | `0O` -> 8 (octal)
/// - `0x` -> 16 (hex)
///
/// `None` is returned if the characters don't match any of the above.
pub fn identify_radix(str: &str) -> Option<u32> {
    if str.starts_with("0x") {
        Some(16)
    } else if str.starts_with("0b") {
        Some(2)
    } else if str.starts_with("0o") || str.starts_with("0O") {
        Some(8)
    } else {
        None
    }
}
