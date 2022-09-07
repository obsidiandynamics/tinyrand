//! Seeding from the system clock.

use std::time::SystemTime;
use tinyrand::{Rand, rand128};
use tinyrand::rand128::Rand128;

/// Derives a seed from the system clock by `XOR`ing the upper 64 bits of the nanosecond timestamp
/// with the lower 64 bits.
///
/// # Examples
/// ```
/// use tinyrand::{StdRand, Rand, Seeded};
/// use tinyrand_std::ClockSeed;
/// let mut seed = ClockSeed::default();
///
/// let mut rand = StdRand::seed(seed.next_u64());
/// println!("{}", rand.next_u64());
/// ```
#[derive(Default)]
pub struct ClockSeed;

impl Rand for ClockSeed {
    fn next_u16(&mut self) -> u16 {
        rand128::next_u16(self)
    }

    fn next_u32(&mut self) -> u32 {
        rand128::next_u32(self)
    }

    fn next_u64(&mut self) -> u64 {
        let time = self.next_u128();
        ((time >> 64) ^ time) as u64
    }

    fn next_u128(&mut self) -> u128 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}

impl Rand128 for ClockSeed {}

#[cfg(test)]
mod tests;