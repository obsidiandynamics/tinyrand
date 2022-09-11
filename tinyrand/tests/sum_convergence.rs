//! Conducts a series of trials on a [`Rand`] with a different (randomly chosen)
//! integer generation range on each trial. Within each trial, H0 asserts that the source is random. (I.e.,
//! the sum of the sampled values falls within a statistically acceptable range.)
///
/// Each trial computes the sum of a set of values drawn from a scaled uniform distribution. The Gaussian distribution
/// is used as an [approximation of the Irwin-Hall distribution](https://en.wikipedia.org/wiki/Irwin%E2%80%93Hall_distribution#Approximating_a_Normal_distribution),
/// with the unscaled mean and variance parameters set to _n_/2 and _n_/12 respectively.

pub mod stats;

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use std::ops::Range;
use statrs::distribution::ContinuousCDF;
use tinyrand::{Counter, RandRange, Seeded, Wyrand, Xorshift};
use crate::stats::{holm_bonferroni_seq_correction, Rejection};

#[test]
fn sum_convergence_wyrand() {
    sum_convergence::<Wyrand>(0, Options::default()).unwrap();
}

#[test]
fn sum_convergence_wyrand_lag_1() {
    sum_convergence::<Wyrand>(1, Options::default()).unwrap();
}

#[test]
fn sum_convergence_wyrand_lag_2() {
    sum_convergence::<Wyrand>(2, Options::default()).unwrap();
}

#[test]
fn sum_convergence_wyrand_lag_4() {
    sum_convergence::<Wyrand>(4, Options::default()).unwrap();
}

#[test]
fn sum_convergence_xorshift() {
    sum_convergence::<Xorshift>(0, Options::default()).unwrap();
}

#[test]
fn sum_convergence_xorshift_lag_1() {
    sum_convergence::<Xorshift>(1, Options::default()).unwrap();
}

#[test]
fn sum_convergence_xorshift_lag_2() {
    sum_convergence::<Xorshift>(2, Options::default()).unwrap();
}

#[test]
fn sum_convergence_xorshift_lag_4() {
    sum_convergence::<Xorshift>(4, Options::default()).unwrap();
}

#[test]
fn sum_convergence_counter_should_reject() {
    assert!(sum_convergence::<Counter>(0, Options::default()).is_err());
}

#[test]
fn sum_convergence_counter_should_reject_lag_1() {
    assert!(sum_convergence::<Counter>(1, Options::default()).is_err());
}

/// Options for conducting multiple trials.
#[derive(Debug)]
pub struct Options {
    /// Number of randomised trials.
    pub trials: u16,

    // Experiments per trial.
    pub iters: u32,

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
            trials: 100,
            iters: 1_000,
            significance_level: 0.025,
        }
    }
}

fn sum_convergence<S: Seeded>(lag: u8, opts: Options) -> Result<(), Vec<Rejection>>
where
    S::R: RandRange<u64>,
{
    opts.validate();
    let mut control_rng = StdRng::seed_from_u64(0);
    let dist_mean = opts.iters as f64 / 2.0;
    let dist_std_dev = (opts.iters as f64 / 12.0).sqrt();
    let dist = statrs::distribution::Normal::new(dist_mean, dist_std_dev).unwrap();

    holm_bonferroni_seq_correction(opts.significance_level, opts.trials, || {
        let seed = control_rng.next_u64();
        let mut rand = S::seed(seed);
        let range = generate_range_for_test(&mut control_rng);

        let sum = (0..opts.iters)
            .map(|_| {
                for _ in 0..lag {
                    rand.next_range(range.clone());
                }
                u128::from(rand.next_range(range.clone()))
            })
            .sum::<u128>();

        let span = (range.end - range.start) as f64;
        let scaled_sum = (sum - u128::from(range.start) * u128::from(opts.iters)) as f64 / span;
        let distance = (scaled_sum - dist_mean).abs();
        let prob_within_distance_from_mean = dist.cdf(dist_mean + distance) - dist.cdf(dist_mean - distance);
        let p_value = 1.0 - prob_within_distance_from_mean;
        p_value
    })
}

fn generate_range_for_test(rng: &mut StdRng) -> Range<u64> {
    let start = rng.next_u64();
    let mut end = 0;
    while end <= start {
        end = rng.next_u64();
    }
    start..end
}
