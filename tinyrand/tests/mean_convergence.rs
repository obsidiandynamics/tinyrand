//! Conducts a series of trials on a [`Rand`] with a different (randomly chosen)
//! integer generation range on each trial. Within each trial, H0 asserts that the source is random. (I.e.,
//! the sum of the samples falls within a statistically acceptable interval.)
//! Multiple trials are run using Bonferroni correction to depress
//! the Type I error rate. I.e., even an ideal random source, subjected to sufficient number
//! of trials, will fail some of them. The significance level is, therefore, scaled to minimise
//! false rejections.

pub mod stats;

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use std::ops::Range;
use statrs::distribution::ContinuousCDF;
use tinyrand::{Counter, RandRange, Seeded, Wyrand, Xorshift};
use crate::stats::{bonferroni_correction, Options, Rejection};

#[test]
fn mean_convergence_wyrand() {
    mean_convergence::<Wyrand>(mean_convergence_options()).unwrap();
}

#[test]
fn mean_convergence_xorshift() {
    mean_convergence::<Xorshift>(mean_convergence_options()).unwrap();
}

#[test]
fn mean_convergence_counter_should_reject() {
    assert!(mean_convergence::<Counter>(mean_convergence_options()).is_err());
}

fn mean_convergence_options() -> Options {
    Options {
        trials: 100,
        iters: 10_000,
        significance_level: 0.25,
    }
}

/// Runs a series of mean convergence trials using Bonferroni correction to depress the Type I error rate.
///
/// Each trial obtains the mean of a set of samples drawn from a scaled uniform distribution. The sum
/// of the values is expected to fall within a statistically acceptable range. The Gaussian distribution
/// is used as an [approximation of the Irwin-Hall distribution](https://en.wikipedia.org/wiki/Irwin%E2%80%93Hall_distribution#Approximating_a_Normal_distribution),
/// with the unscaled mean and variance
/// parameters set to _n_/2 and _n_/12 respectively, where _n_ is the number of samples.
fn mean_convergence<S: Seeded>(opts: Options) -> Result<(), Vec<Rejection>>
where
    S::R: RandRange<u64>,
{
    opts.validate();
    let mut control_rng = StdRng::seed_from_u64(0);
    let dist_mean = opts.iters as f64 / 2.0;
    let dist_std_dev = (opts.iters as f64 / 12.0).sqrt();
    let dist = statrs::distribution::Normal::new(dist_mean, dist_std_dev).unwrap();

    bonferroni_correction(opts.significance_level, opts.trials, || {
        let seed = control_rng.next_u64();
        let mut rand = S::seed(seed);
        let range = generate_range_for_test(&mut control_rng);

        let sum = (0..opts.iters)
            .map(|_| u128::from(rand.next_range(range.clone())))
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
