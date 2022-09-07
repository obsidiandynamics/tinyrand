//! [Wyrand](https://github.com/wangyi-fudan/wyhash/blob/master/Modern%20Non-Cryptographic%20Hash%20Function%20and%20Pseudorandom%20Number%20Generator.pdf) RNG.

use crate::{Rand, rand64, Seeded};
use crate::rand64::Rand64;

#[derive(Default)]
pub struct Wyrand(u64);

impl Rand for Wyrand {
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
        self.0 = self.0.wrapping_add(0xA076_1D64_78BD_642F);
        let r = u128::from(self.0) * u128::from(self.0 ^ 0xE703_7ED1_A0B4_28DB);
        (r as u64) ^ (r >> 64) as u64
    }

    #[inline(always)]
    fn next_u128(&mut self) -> u128 {
        rand64::next_u128(self)
    }
}

impl Rand64 for Wyrand {}

impl Seeded for Wyrand {
    type R = Wyrand;

    #[inline(always)]
    fn seed(seed: u64) -> Self::R {
        Self(seed)
    }
}

#[cfg(test)]
mod tests;