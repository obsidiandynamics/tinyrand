//! Conducts a series of Bernoulli trials on a [`Rand`] with a different (randomly chosen)
//! weighting on each trial, simulating a run of coin flips. Within each trial,
//! H0 asserts that the source is random. (I.e.,
//! the number of 'heads' falls within a statistically acceptable interval.)

pub mod stats;

use crate::stats::{holm_bonferroni_seq_correction, integrate_binomial, Rejection};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use tinyrand::{Counter, Probability, Rand, RandRange, Seeded, SplitMix, Wyrand, Xorshift};

#[test]
fn coin_flip_splitmix() {
    coin_flip::<SplitMix>(Options::default()).unwrap();
}

#[test]
fn coin_flip_wyrand() {
    coin_flip::<Wyrand>(Options::default()).unwrap();
}

#[test]
fn coin_flip_xorshift() {
    coin_flip::<Xorshift>(Options::default()).unwrap();
}

#[test]
fn coin_flip_counter_should_reject() {
    assert!(coin_flip::<Counter>(Options::default()).is_err());
}

/// Options for conducting multiple trials.
#[derive(Debug)]
pub struct Options {
    /// Number of randomised trials.
    pub trials: u16,

    // Experiments per trial.
    pub iters: u16,

    // Significance level to reject H0 (stream is random). The higher the significance level, the more likely
    // H1 (stream is nonrandom) is accepted.
    pub significance_level: f64,
}

impl Options {
    /// Checks that the options are valid.
    pub fn validate(&self) {
        assert!(self.trials > 0);
        assert!(self.iters > 0);
        assert!(self.significance_level >= f64::EPSILON);
        assert!(self.significance_level <= 1.0 - f64::EPSILON);
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            trials: 1000,
            iters: 30,
            significance_level: 0.25,
        }
    }
}

fn coin_flip<S: Seeded>(opts: Options) -> Result<(), Vec<Rejection>>
where
    S::R: RandRange<u64>,
{
    opts.validate();
    let mut control_rng = StdRng::seed_from_u64(0);

    holm_bonferroni_seq_correction(opts.significance_level, opts.trials, || {
        let seed = control_rng.next_u64();
        let mut rand = S::seed(seed);
        let weight = generate_weight_for_test(&mut control_rng);
        let prob_heads = Probability::new(weight);
        let mut heads = 0;
        for _ in 0..opts.iters {
            if rand.next_bool(prob_heads) {
                heads += 1;
            }
        }
        let run_within_prob = integrate_binomial(opts.iters, weight, heads);
        let p_value = 1.0 - run_within_prob;
        p_value
    })
}

fn generate_weight_for_test(rng: &mut StdRng) -> f64 {
    let p = rng.next_u64() as f64 / u64::MAX as f64;
    assert!(p >= 0.0);
    assert!(p <= 1.0);
    p
}
