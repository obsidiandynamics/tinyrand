//! Traits describing a basic RNG and various capability extenders.
//! The traits and RNGs here do not need stdlib to work, nor do they need `alloc`.

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
pub type StdRand = Wyrand;

/// A minimal specification of a random number generator.
///
/// Implementers must, at minimum, provide a working [`next_u64`]. The rest of the methods will
/// be derived. The default implementations either truncate the generated number (`u16`, `u32`)
/// or generate several numbers and splice the outputs (`u128`). Implementers may provide more 
/// efficient versions for `u16`, `u32` and `u128` generators, overriding the defaults.
pub trait Rand {
    /// Returns the next random `u16`.
    #[inline(always)]
    fn next_u16(&mut self) -> u16 {
        self.next_u64() as u16
    }

    /// Returns the next random `u32`.
    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    /// Returns the next random `u64`.
    fn next_u64(&mut self) -> u64;

    /// Returns the next random `u128`.
    #[inline(always)]
    fn next_u128(&mut self) -> u128 {
        u128::from(self.next_u64()) << 64 | u128::from(self.next_u64())
    }

    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    fn next_usize(&mut self) -> usize {
        self.next_u16() as usize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn next_usize(&mut self) -> usize {
        self.next_u32() as usize
    }

    /// Returns the next random `usize`.
    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn next_usize(&mut self) -> usize {
        self.next_u64() as usize
    }

    #[cfg(target_pointer_width = "128")]
    #[inline(always)]
    fn next_usize(&mut self) -> usize {
        self.next_u128() as usize
    }

    /// Returns a `bool` with a probability `p` of being true.
    ///
    /// # Example
    /// ```
    /// use tinyrand::{StdRand, Probability, Rand};
    /// let mut rng = StdRand::default();
    /// println!("{}", rng.next_bool(Probability::new(1.0 / 3.0)));
    /// ```
    #[inline(always)]
    fn next_bool(&mut self, p: Probability) -> bool {
        #[allow(clippy::cast_precision_loss)]
        #[allow(clippy::cast_sign_loss)]
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
#[derive(Clone, Copy, PartialEq, Debug)]
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

    /// Creates a new [`Probability`] value, without checking the bounds.
    ///
    /// # Safety
    /// If a probability is created outside the range \[0, 1\], its behaviour with an
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
    type R: Rand;

    /// Creates a new [`Rand`] instance from the given seed.
    fn seed(seed: u64) -> Self::R;
}

pub trait RandLim<N> {
    /// Generates a random number in `0..N`.
    fn next_lim(&mut self, lim: N) -> N;
}

impl<R: Rand> RandLim<u16> for R {
    #[inline(always)]
    fn next_lim(&mut self, lim: u16) -> u16 {
        assert_ne!(0, lim, "zero limit");
        let mut full = u32::from(self.next_u16()) * u32::from(lim);
        let mut low = full as u16;
        if low < lim {
            let cutoff = lim.wrapping_neg() % lim;
            while low < cutoff {
                full = u32::from(self.next_u16()) * u32::from(lim);
                low = full as u16;
            }
        }
        (full >> 16) as u16
    }
}

impl<R: Rand> RandLim<u32> for R {
    #[inline(always)]
    fn next_lim(&mut self, lim: u32) -> u32 {
        assert_ne!(0, lim, "zero limit");
        let mut full = u64::from(self.next_u32()) * u64::from(lim);
        let mut low = full as u32;
        if low < lim {
            let cutoff = lim.wrapping_neg() % lim;
            while low < cutoff {
                full = u64::from(self.next_u32()) * u64::from(lim);
                low = full as u32;
            }
        }
        (full >> 32) as u32
    }
}

impl<R: Rand> RandLim<u64> for R {
    #[inline(always)]
    fn next_lim(&mut self, lim: u64) -> u64 {
        assert_ne!(0, lim, "zero limit");
        let mut full = u128::from(self.next_u64()) * u128::from(lim);
        let mut low = full as u64;
        if low < lim {
            let cutoff = lim.wrapping_neg() % lim;
            while low < cutoff {
                full = u128::from(self.next_u64()) * u128::from(lim);
                low = full as u64;
            }
        }
        (full >> 64) as u64
    }
}

impl<R: Rand> RandLim<u128> for R {
    #[inline(always)]
    fn next_lim(&mut self, lim: u128) -> u128 {
        assert_ne!(0, lim, "zero limit");
        if lim <= u128::from(u64::MAX) {
            u128::from(self.next_lim(lim as u64))
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

impl<R: Rand> RandLim<usize> for R {
    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    fn next_lim(&mut self, lim: usize) -> usize {
        self.next_lim(lim as u16) as usize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn next_lim(&mut self, lim: usize) -> usize {
        self.next_lim(lim as u32) as usize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn next_lim(&mut self, lim: usize) -> usize {
        self.next_lim(lim as u64) as usize
    }

    #[cfg(target_pointer_width = "128")]
    #[inline(always)]
    fn next_lim(&mut self, lim: usize) -> usize {
        self.next_lim(lim as u128) as usize
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

impl<R: Rand> RandRange<u16> for R {
    #[inline(always)]
    fn next_range(&mut self, range: Range<u16>) -> u16 {
        assert!(!range.is_empty(), "empty range");
        let span = range.end - range.start;
        range.start + self.next_lim(span)
    }
}

impl<R: Rand> RandRange<u32> for R {
    #[inline(always)]
    fn next_range(&mut self, range: Range<u32>) -> u32 {
        assert!(!range.is_empty(), "empty range");
        let span = range.end - range.start;
        range.start + self.next_lim(span)
    }
}

impl<R: Rand> RandRange<u64> for R {
    #[inline(always)]
    fn next_range(&mut self, range: Range<u64>) -> u64 {
        assert!(!range.is_empty(), "empty range");
        let span = range.end - range.start;
        range.start + self.next_lim(span)
    }
}

impl<R: Rand> RandRange<u128> for R {
    #[inline(always)]
    fn next_range(&mut self, range: Range<u128>) -> u128 {
        assert!(!range.is_empty(), "empty range");
        let span = range.end - range.start;
        let random = self.next_lim(span);
        range.start + random
    }
}

impl<R: Rand> RandRange<usize> for R {
    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    fn next_range(&mut self, range: Range<usize>) -> usize {
        self.next_range(range.start as u16..range.end as u16) as usize
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn next_range(&mut self, range: Range<usize>) -> usize {
        self.next_range(range.start as u32..range.end as u32) as usize
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn next_range(&mut self, range: Range<usize>) -> usize {
        self.next_range(range.start as u64..range.end as u64) as usize
    }

    #[cfg(target_pointer_width = "128")]
    #[inline(always)]
    fn next_range(&mut self, range: Range<usize>) -> usize {
        self.next_range(range.start as u128..range.end as u128) as usize
    }
}

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
mod tests;