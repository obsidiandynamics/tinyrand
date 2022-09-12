//! Conducts a series of Bernoulli trials on a [`Rand`] by masking the value of a single bit,
//! verifying that the number of times the bit is set to 1 is within the expected range. For
//! each subsequent trial, the mask is shifted by one to the left and the hypothesis is retested.

pub mod stats;

use crate::stats::{holm_bonferroni_seq_correction, integrate_binomial, Rejection};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use tinyrand::{Counter, Rand, RandRange, Seeded, SplitMix, Wyrand, Xorshift};

#[test]
fn bit_flip_splitmix() {
    bit_flip::<SplitMix>(Options::default()).unwrap();
}

#[test]
fn bit_flip_wyrand() {
    bit_flip::<Wyrand>(Options::default()).unwrap();
}

#[test]
fn bit_flip_xorshift() {
    bit_flip::<Xorshift>(Options::default()).unwrap();
}

#[test]
fn bit_flip_counter_should_reject() {
    assert!(bit_flip::<Counter>(Options::default()).is_err());
}

#[test]
fn bit_flip_faulty_should_reject() {
    assert!(bit_flip::<LsbFaultyRand<Wyrand>>(Options::default()).is_err());
}

/// A faulty RNG that always returns a value with the LSB set to 0. For every other bit,
/// it echoes the value of some other (presumably nonfaulty) RNG.
struct LsbFaultyRand<R: Rand>(R);

impl<R: Rand> Rand for LsbFaultyRand<R> {
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64() & 0xFFFF_FFFF_FFFF_FFFE
    }
}

impl<S: Seeded + Rand> Seeded for LsbFaultyRand<S> {
    type R = LsbFaultyRand<S::R>;

    fn seed(seed: u64) -> Self::R {
        LsbFaultyRand(S::seed(seed))
    }
}

/// Options for conducting multiple trials.
#[derive(Debug)]
pub struct Options {
    /// Number of trial cycles. Each cycle comprises 64 trials (one for each bit of a `u64`).
    pub cycles: u16,

    // Experiments per trial.
    pub iters: u16,

    // Significance level to reject H0 (stream is random). The higher the significance level, the more likely
    // H1 (stream is nonrandom) is accepted.
    pub significance_level: f64,
}

impl Options {
    /// Checks that the options are valid.
    pub fn validate(&self) {
        assert!(self.cycles > 0);
        assert!(self.iters > 0);
        assert!(self.significance_level >= f64::EPSILON);
        assert!(self.significance_level <= 1.0 - f64::EPSILON);
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            cycles: 10,
            iters: 30,
            significance_level: 0.2,
        }
    }
}

fn bit_flip<S: Seeded>(opts: Options) -> Result<(), Vec<Rejection>>
where
    S::R: RandRange<u64>,
{
    opts.validate();
    let mut control_rng = StdRng::seed_from_u64(0);

    let mut trial = 0;
    holm_bonferroni_seq_correction(opts.significance_level, opts.cycles * 64, || {
        let seed = control_rng.next_u64();
        let mut rand = S::seed(seed);
        let mask = 1u64 << (trial % 64);
        trial += 1;
        let mut set_bits = 0;
        for _ in 0..opts.iters {
            if rand.next_u64() & mask > 0  {
                set_bits += 1;
            }
        }
        let run_within_prob = integrate_binomial(opts.iters, 0.5, set_bits);
        let p_value = 1.0 - run_within_prob;
        p_value
    })
}