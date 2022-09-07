//! A template for composing RNGs optimised for 128-bit words.
//!
//! Implementers must, at minimum, provide a working [`Rand::next_u128`]. The rest of the methods may
//! be relegated to the default implementations defined in this module. The default implementations
//! truncate the generated number (`u16`, `u32`, `u64`). Implementers may provide more efficient
//! versions for `u16`, `u32` and `u64` generators, overriding the defaults.

use crate::Rand;

/// Marker trait that indicates that a [`Rand`] implementation is optimised for 128-bit words.
pub trait Rand128: Rand {}

/// Returns the next random `u16`.
#[inline(always)]
pub fn next_u16(rand: &mut impl Rand128) -> u16 {
    rand.next_u128() as u16
}

/// Returns the next random `u32`.
#[inline(always)]
pub fn next_u32(rand: &mut impl Rand128) -> u32 {
    rand.next_u128() as u32
}

/// Returns the next random `u64`.
#[inline(always)]
pub fn next_u64(rand: &mut impl Rand128) -> u64 { rand.next_u128() as u64}