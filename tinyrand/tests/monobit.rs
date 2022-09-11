//! Conducts a series of trials on a [`Rand`] by counting the number of bits in 32-bit
//! words, obtained by alternating between the MSB and LSB segments of generated `u64`s in separate
//! trials. In each
//! trial, we assume that the values of individual bits are IID with probability of 0.5,
//! verifying that the number of times the bit is set to 1 is within the expected range. For
//! a random source, the number of 1s (and 0s) follows a Bernoulli process.
pub mod stats;

use crate::stats::{holm_bonferroni_seq_correction, integrate_binomial, Rejection};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use tinyrand::{Counter, Rand, RandRange, Seeded, Wyrand, Xorshift};

#[test]
fn monobit_wyrand() {
    monobit::<Wyrand>(Options::default()).unwrap();
}

#[test]
fn monobit_xorshift() {
    monobit::<Xorshift>(Options::default()).unwrap();
}

#[test]
fn monobit_counter_should_reject() {
    assert!(monobit::<Counter>(Options::default()).is_err());
}

/// Options for conducting multiple trials.
#[derive(Debug)]
pub struct Options {
    /// Number of trial cycles. Each cycle comprises two trials (one for the MSB
    /// part of a random `u64` and one for the LSB). A trial comprises 32 experiments.
    pub cycles: u16,

    // Significance level to reject H0 (stream is random). The higher the significance level, the more likely
    // H1 (stream is nonrandom) is accepted.
    pub significance_level: f64,
}

impl Options {
    /// Checks that the options are valid.
    pub fn validate(&self) {
        assert!(self.cycles > 0);
        assert!(self.significance_level >= f64::EPSILON);
        assert!(self.significance_level <= 1.0 - f64::EPSILON);
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            cycles: 100,
            significance_level: 0.25,
        }
    }
}

fn monobit<S: Seeded>(opts: Options) -> Result<(), Vec<Rejection>>
where
    S::R: RandRange<u64>,
{
    opts.validate();
    let mut control_rng = StdRng::seed_from_u64(0);

    let mut trial = 0;
    holm_bonferroni_seq_correction(opts.significance_level, opts.cycles * 2, || {
        let seed = control_rng.next_u64();
        let mut rand = S::seed(seed);
        let word = if trial % 1 == 0 {
            rand.next_u64() as u32
        } else {
            (rand.next_u64() >> 32) as u32
        };
        trial += 1;
        let ones = count_ones(word);
        let run_within_prob = integrate_binomial(32, 0.5, u16::from(ones));
        let p_value = 1.0 - run_within_prob;
        p_value
    })
}

fn count_ones(word: u32) -> u8 {
    let mut set_bits = 0;
    for shift in 0..32 {
        let mask = 0x1 << shift;
        if word & mask > 0  {
            set_bits += 1;
        }
    }
    set_bits
}

#[test]
fn self_test_count_ones() {
    assert_eq!(0, count_ones(0x0000_0000));
    assert_eq!(1, count_ones(0x0000_0001));
    assert_eq!(1, count_ones(0x0000_0010));
    assert_eq!(1, count_ones(0x1000_0000));
    assert_eq!(2, count_ones(0x1000_0001));
}