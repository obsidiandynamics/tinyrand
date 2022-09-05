//! Seeding from the system clock.

use std::time::SystemTime;
use tinyrand::Rand;

/// Derives a seed from the system clock by XORing the upper 64 bits of the nanosecond timestamp
/// with the lower 64 bits.
///
/// # Examples
/// ```
/// use tinyrand::{DefaultRand, Rand, Seeded};
/// use tinyrand_std::ClockSeed;
/// let mut seed = ClockSeed::default();
///
/// let mut rand = DefaultRand::seed(seed.next_u64());
/// println!("{}", rand.next_u64());
/// ```
#[derive(Default)]
pub struct ClockSeed;

impl Rand for ClockSeed {
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        ((time >> 64) ^ time) as u64
    }
}

#[cfg(test)]
mod tests;