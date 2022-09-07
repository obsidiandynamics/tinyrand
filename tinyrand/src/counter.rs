//! A wrap-around 64-bit counter. Useful for seeding and testing.

use crate::{Rand, rand64, Seeded};
use crate::rand64::Rand64;

/// A wrap-around counter.
///
/// # Examples
/// ```
/// use tinyrand::{Counter, Rand};
/// let mut rand = Counter::default();
/// assert_eq!(0, rand.next_u64());
/// assert_eq!(1, rand.next_u64());
/// ```
#[derive(Debug, Default)]
pub struct Counter(u64);

impl Counter {
    #[inline(always)]
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }
}

impl Rand for Counter {
    fn next_u16(&mut self) -> u16 {
        rand64::next_u16(self)
    }

    fn next_u32(&mut self) -> u32 {
        rand64::next_u32(self)
    }

    fn next_u64(&mut self) -> u64 {
        let current = self.0;
        self.0 = current.wrapping_add(1);
        current
    }

    fn next_u128(&mut self) -> u128 {
        rand64::next_u128(self)
    }
}

impl Rand64 for Counter {}

impl Seeded for Counter {
    type R = Counter;

    #[inline(always)]
    fn seed(seed: u64) -> Self::R {
        Self(seed)
    }
}

#[cfg(test)]
mod tests;