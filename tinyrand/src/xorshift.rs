//! [Xorshift](https://en.wikipedia.org/wiki/Xorshift) RNG.

use crate::{Rand, Seeded};

pub struct Xorshift(u64);

impl Default for Xorshift {
    #[inline(always)]
    fn default() -> Self {
        Self(1)
    }
}

impl Rand for Xorshift {
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        let mut s = self.0;
        s ^= s << 13;
        s ^= s >> 7;
        self.0 = s;
        s ^= s << 17;
        s
    }
}

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