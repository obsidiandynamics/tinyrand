//! [Wyrand](https://github.com/wangyi-fudan/wyhash/blob/master/Modern%20Non-Cryptographic%20Hash%20Function%20and%20Pseudorandom%20Number%20Generator.pdf) RNG.

use crate::{Rand, Seeded};

#[derive(Default)]
pub struct Wyrand(u64);

impl Rand for Wyrand {
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0xA076_1D64_78BD_642F);
        let r = u128::from(self.0) * u128::from(self.0 ^ 0xE703_7ED1_A0B4_28DB);
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