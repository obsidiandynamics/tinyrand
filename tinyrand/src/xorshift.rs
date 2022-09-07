//! [Xorshift](https://en.wikipedia.org/wiki/Xorshift) RNG.

use crate::{Rand, rand64, Seeded};
use crate::rand64::Rand64;

pub struct Xorshift(u64);

impl Default for Xorshift {
    #[inline(always)]
    fn default() -> Self {
        Self(1)
    }
}

impl Rand for Xorshift {
    #[inline(always)]
    fn next_u16(&mut self) -> u16 {
        rand64::next_u16(self)
    }

    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        rand64::next_u32(self)
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        let mut s = self.0;
        s ^= s << 13;
        s ^= s >> 7;
        self.0 = s;
        s ^= s << 17;
        s
    }

    #[inline(always)]
    fn next_u128(&mut self) -> u128 {
        rand64::next_u128(self)
    }
}

impl Rand64 for Xorshift {}

impl Seeded for Xorshift {
    type R = Xorshift;

    #[inline(always)]
    fn seed(seed: u64) -> Self::R {
        // a zero seed disables Xorshift, rendering it (effectively) a constant; hence, we avoid it
        Self(if seed == 0 { u64::MAX >> 1 } else { seed })
    }
}

#[cfg(test)]
mod tests;