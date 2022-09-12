//! [`SplitMix`](https://dl.acm.org/doi/10.1145/2660193.2660195) RNG.

use crate::{Rand, Seeded};

pub struct SplitMix(u64);

impl Default for SplitMix {
    #[inline(always)]
    fn default() -> Self {
        Self(1)
    }
}

impl Rand for SplitMix {
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        let mut s = u128::from(self.0);
        s = u128::from((s ^ (s >> 32)) as u64) * 0xff51_afd7_ed55_8ccd;
        self.0 = s as u64;
        s = u128::from((s ^ (s >> 32)) as u64) * 0xc4ce_b9fe_1a85_ec53;
        s as u64
    }
}

impl Seeded for SplitMix {
    type R = SplitMix;

    #[inline(always)]
    fn seed(seed: u64) -> Self::R {
        // a zero seed disables SplitMix, rendering it (effectively) a constant; hence, we avoid it
        Self(if seed == 0 { u64::MAX >> 1 } else { seed })
    }
}

#[cfg(test)]
mod tests;