//! A wrap-around 64-bit counter. Useful for seeding and testing.

use crate::{Rand, Seeded};

/// A wrap-around counter.
///
/// # Examples
/// ```
/// use tinyrand::{Counter, Rand};
/// let mut rand = Counter::default();
/// assert_eq!(0, rand.next_u64());
/// assert_eq!(1, rand.next_u64());
/// ```
#[derive(Debug)]
pub struct Counter(u64);

impl Counter {
    #[inline(always)]
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }
}

impl Default for Counter {
    #[inline(always)]
    fn default() -> Self {
        Self(0)
    }
}

impl Rand for Counter {
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        let current = self.0;
        self.0 = current.wrapping_add(1);
        current
    }
}

impl Seeded for Counter {
    type Rng = Counter;

    #[inline(always)]
    fn seed(seed: u64) -> Self::Rng {
        Self(seed)
    }
}

#[cfg(test)]
mod tests;