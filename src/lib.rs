//! Traits describing a basic RNG and various capability extenders.

#![no_std]

pub mod counter;
pub mod duration;
pub mod mock;
pub mod xorshift;
pub mod wyrand;

pub use counter::Counter;
pub use mock::Mock;
pub use wyrand::Wyrand;
pub use xorshift::Xorshift;

use core::ops::Range;

/// The default/recommended [`Rand`] implementation.
pub type DefaultRand = Wyrand;

/// A minimal specification of a 64-bit random number generator.
pub trait Rand {
    /// Returns the next random `u64`.
    fn next_u64(&mut self) -> u64;

    /// Returns the next random `u128`.
    #[inline(always)]
    fn next_u128(&mut self) -> u128 {
        (self.next_u64() as u128) << 64 | (self.next_u64() as u128)
    }

    /// Returns a `bool` with a probability `p` of being true.
    ///
    /// # Example
    /// ```
    /// use tinyrand::{DefaultRand, Probability, Rand};
    /// let mut rng = DefaultRand::default();
    /// println!("{}", rng.next_bool(Probability::new(1.0 / 3.0)));
    /// ```
    #[inline(always)]
    fn next_bool(&mut self, p: Probability) -> bool {
        let cutoff = (p.0 * u64::MAX as f64) as u64;
        let mut next = self.next_u64();
        if next == u64::MAX {
            // guarantees that gen_bool(p=1.0) is never true
            next = u64::MAX - 1;
        }
        next < cutoff
    }
}

/// Represents a probability in the range \[0, 1\].
#[derive(Clone, Copy)]
pub struct Probability(f64);

impl Probability {
    /// Creates a new [`Probability`] value, bounded in the range \[0, 1\].
    ///
    /// # Example
    /// ```
    /// use tinyrand::Probability;
    /// let p = Probability::new(0.25);
    /// assert_eq!(0.25, p.into());
    /// ```
    ///
    /// # Panics
    /// If `p < 0` or `p > 1`.
    #[inline(always)]
    pub fn new(p: f64) -> Self {
        assert!(p >= 0f64, "p ({p}) cannot be less than 0");
        assert!(p <= 1f64, "p ({p}) cannot be greater than 1");
        Self(p)
    }

    /// Creates a new [`Probability`] value, without checking the bounds. If a
    /// probability is created outside the range \[0, 1\], its behaviour with an
    /// RNG is undefined.
    #[inline(always)]
    pub const unsafe fn new_unchecked(p: f64) -> Self {
        Self(p)
    }
}

impl From<Probability> for f64 {
    #[inline(always)]
    fn from(p: Probability) -> Self {
        p.0
    }
}

impl From<f64> for Probability {
    #[inline(always)]
    fn from(p: f64) -> Self {
        Probability::new(p)
    }
}

/// The means for seeding an RNG.
pub trait Seeded {
    type Rng: Rand;

    /// Creates a new [`Rand64`] instance from the given seed.
    fn seed(seed: u64) -> Self::Rng;
}

pub trait RandLim<N> {
    /// Generates a random number in `0..N`.
    fn next_lim(&mut self, lim: N) -> N;
}

impl<R: Rand> RandLim<u64> for R {
    #[inline(always)]
    fn next_lim(&mut self, lim: u64) -> u64 {
        let mut full = self.next_u64() as u128 * lim as u128;
        let mut low = full as u64;
        if low < lim {
            let cutoff = lim.wrapping_neg() % lim;
            while low < cutoff {
                full = self.next_u64() as u128 * lim as u128;
                low = full as u64;
            }
        }
        (full >> 64) as u64
    }
}

impl<R: Rand> RandLim<u128> for R {
    #[inline(always)]
    fn next_lim(&mut self, lim: u128) -> u128 {
        if lim <= u64::MAX as u128 {
            self.next_lim(lim as u64) as u128
        } else {
            let cutoff = cutoff_u128(lim);
            loop {
                let rand = self.next_u128();
                if rand <= cutoff {
                    return rand % lim;
                }
            }
        }
    }
}

#[inline(always)]
fn cutoff_u128(lim: u128) -> u128 {
    let overhang = (u128::MAX - lim + 1) % lim;
    u128::MAX - overhang
}

pub trait RandRange<N> {
    /// Generates a random number in the given range.
    fn next_range(&mut self, range: Range<N>) -> N;
}

impl<R: Rand> RandRange<u64> for R {
    #[inline(always)]
    fn next_range(&mut self, range: Range<u64>) -> u64 {
        if range.is_empty() {
            return range.start;
        }
        let span = range.end - range.start;
        range.start + self.next_lim(span)
    }
}

impl<R: Rand> RandRange<u128> for R {
    #[inline(always)]
    fn next_range(&mut self, range: Range<u128>) -> u128 {
        if range.is_empty() {
            return range.start;
        }
        let span = range.end - range.start;
        let random = self.next_lim(span);
        range.start + random
    }
}

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
mod tests;