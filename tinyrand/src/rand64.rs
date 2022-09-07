//! A template for composing RNGs optimised for 64-bit words.
//!
//! Implementers must, at minimum, provide a working [`Rand::next_u64`]. The rest of the methods may
//! be relegated to the default implementations defined in this module. The default implementations
//! either truncate the generated number (`u16`, `u32`) or generate several numbers and splice
//! the outputs (`u128`). Implementers may provide more efficient versions for `u16`, `u32` and
//! `u128` generators, overriding the defaults.

use crate::Rand;

/// Marker trait that indicates that a [`Rand`] implementation is optimised for 64-bit words.
pub trait Rand64: Rand {}

/// Returns the next random `u16`.
#[inline(always)]
pub fn next_u16(rand: &mut impl Rand64) -> u16 {
    rand.next_u64() as u16
}

/// Returns the next random `u32`.
#[inline(always)]
pub fn next_u32(rand: &mut impl Rand64) -> u32 {
    rand.next_u64() as u32
}

/// Returns the next random `u128`.
#[inline(always)]
pub fn next_u128(rand: &mut impl Rand64) -> u128 {
    u128::from(rand.next_u64()) << 64 | u128::from(rand.next_u64())
}