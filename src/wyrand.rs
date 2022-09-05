//! [Wyrand](https://github.com/wangyi-fudan/wyhash/blob/master/Modern%20Non-Cryptographic%20Hash%20Function%20and%20Pseudorandom%20Number%20Generator.pdf) RNG.

use crate::{Rand, Seeded};

pub struct Wyrand(u64);

impl Default for Wyrand {
    #[inline(always)]
    fn default() -> Self {
        Self(0)
    }
}

impl Rand for Wyrand {
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0xA0761D6478BD642F);
        let r = self.0 as u128 * (self.0 ^ 0xE7037ED1A0B428DB) as u128;
        (r as u64) ^ (r >> 64) as u64
    }
}

impl Seeded for Wyrand {
    type Rng = Wyrand;

    #[inline(always)]
    fn seed(seed: u64) -> Self::Rng {
        Self(seed)
    }
}

#[cfg(test)]
mod tests;