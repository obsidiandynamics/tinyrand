//! Conducts a series of trials on a [`Rand`] with a different (randomly chosen)
//! integer generation range on each trial. Within each trial, H0 asserts that the source is random. (I.e.,
//! the sum of the samples falls within a statistically acceptable interval.)
//! Multiple trials are run using Bonferroni correction to depress
//! the Type I error rate. I.e., even an ideal random source, subjected to sufficient number
//! of trials, will fail some of them. The significance level is, therefore, scaled to minimise
//! false rejections.

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use std::any;
use std::ops::Range;
use statrs::distribution::ContinuousCDF;
use tinyrand::{Counter, RandRange, Seeded, Wyrand, Xorshift};

#[test]
fn mean_convergence_wyrand() {
    mean_convergence_bc::<Wyrand>(Options::default()).unwrap();
}

#[test]
fn mean_convergence_xorshift() {
    mean_convergence_bc::<Xorshift>(Options::default()).unwrap();
}

#[test]
fn mean_convergence_counter_should_reject() {
    assert!(mean_convergence_bc::<Counter>(Options::default()).is_err());
}

#[derive(Debug)]
struct Options {
    /// Number of randomised trials.
    trials: u32,

    // Experiments per trial.
    iters: u16,

    // Significance level to reject H0 (stream is random). The higher the significance level, the more likely
    // H1 (stream is nonrandom) is accepted.
    significance_level: f64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            trials: 100,
            iters: 10_000,
            significance_level: 0.25,
        }
    }
}

impl Options {
    fn validate(&self) {
        assert!(self.trials > 0);
        assert!(self.iters > 0);
        assert!(self.significance_level >= f64::EPSILON);
        assert!(self.significance_level <= 1.0 - f64::EPSILON);
    }
}

#[derive(Debug)]
pub struct Rejection {
    /// The significance level at which H0 was rejected.
    pub alpha: f64,

    /// The p-value of the test.
    pub p_value: f64,
}

/// Runs a series of mean convergence trials using Bonferroni correction to depress the Type I error rate.
///
/// Each trial obtains the mean of a set of samples drawn from a scaled uniform distribution. The sum
/// of the values is expected to fall within a statistically acceptable range. The Gaussian distribution
/// is used as an approximation of the Irwin-Hall distribution, with the unscaled mean and variance
/// parameters set to _n_/2 and _n_/12 respectively, where _n_ is the number of samples.
fn mean_convergence_bc<S: Seeded>(opts: Options) -> Result<(), Vec<Rejection>>
where
    S::R: RandRange<u64>,
{
    opts.validate();
    let rand_type = any::type_name::<S>();
    let mut control_rng = StdRng::seed_from_u64(0);
    let alpha = opts.significance_level / f64::from(opts.trials);
    let dist_mean = opts.iters as f64 / 2.0;
    let dist_std_dev = (opts.iters as f64 / 12.0).sqrt();
    // println!("dist_mean={dist_mean}, dist_std_dev={dist_std_dev}");
    let dist = statrs::distribution::Normal::new(dist_mean, dist_std_dev).unwrap();
    let mut rejections = vec![];

    for trial in 0..opts.trials {
        let seed = control_rng.next_u64();
        let mut rand = S::seed(seed);
        let range = generate_range_for_test(&mut control_rng);

        let sum = (0..opts.iters)
            .map(|_| u128::from(rand.next_range(range.clone())))
            .sum::<u128>();
        // let mean = sum / u128::from(opts.iters);

        let span = (range.end - range.start) as f64;
        let scaled_sum = (sum - u128::from(range.start) * u128::from(opts.iters)) as f64 / span;
        let distance = (scaled_sum - dist_mean).abs();
        let prob_within_distance_from_mean = dist.cdf(dist_mean + distance) - dist.cdf(dist_mean - distance);
        let p_value = 1.0 - prob_within_distance_from_mean;
        // println!("{}", dist.cdf(dist_mean + delta));
        // println!("range: {range:?}, sum={sum}, scaled_sum={scaled_sum}, delta={delta}, p_value={p_value}");

        if p_value < alpha {
            println!("[{rand_type} trial {trial}] rejected H0: scaled_sum={scaled_sum}, distance={distance}, p={p_value} < {alpha}");
            rejections.push(p_value);
        }
    }

    println!(
        "[{rand_type}] rejections/trials: {}/{}",
        rejections.len(),
        opts.trials
    );

    if rejections.is_empty() {
        Ok(())
    } else {
        let rejections = rejections
            .into_iter()
            .map(|p_value| Rejection { alpha, p_value })
            .collect();
        Err(rejections)
    }
}

fn generate_range_for_test(rng: &mut StdRng) -> Range<u64> {
    let start = rng.next_u64();
    let mut end = 0;
    while end <= start {
        end = rng.next_u64();
    }
    start..end
}