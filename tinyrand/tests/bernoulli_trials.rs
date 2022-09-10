//! Conducts a series of Bernoulli trials on a [`Rand`] with a different (randomly assigned)
//! weighting on each trial. Within each trial, H0 asserts that the source is random.
//! Counts the number of rejected trials and asserts that the number of rejections does not
//! exceed the maximum expected number of rejections for a random source. I.e., even the best
//! random source, subjected to sufficient number of trials, will fail some of them. The
//! number of failed trials should, therefore, remain within the confidence level.

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use std::any;
use tinyrand::{Counter, Probability, Rand, RandRange, Seeded, Wyrand, Xorshift};

#[test]
fn bernoulli_trials_wyrand() {
    bernoulli_trials_bc::<Wyrand>(Options::default()).unwrap();
}

#[test]
fn bernoulli_trials_xorshift() {
    bernoulli_trials_bc::<Xorshift>(Options::default()).unwrap();
}

#[test]
fn bernoulli_trials_counter_should_reject() {
    assert!(bernoulli_trials_bc::<Counter>(Options::default()).is_err());
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
            trials: 1_000,
            iters: 30,
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

/// Runs a series of Bernoulli trials using Bonferroni correction to depress the Type I error rate.
fn bernoulli_trials_bc<S: Seeded>(opts: Options) -> Result<(), Vec<Rejection>>
where
    S::R: RandRange<u64>,
{
    opts.validate();
    let rand_type = any::type_name::<S>();
    let mut control_rng = StdRng::seed_from_u64(0);
    let alpha = opts.significance_level / f64::from(opts.trials);
    let mut rejections = vec![];

    for trial in 0..opts.trials {
        let seed = control_rng.next_u64();
        let mut rand = S::seed(seed);
        let weight = generate_weight_for_test(&mut control_rng);
        let prob = Probability::new(weight);
        let mut heads = 0;
        for _ in 0..opts.iters {
            if rand.next_bool(prob) {
                heads += 1;
            }
        }
        let run_within_prob = integrate_outcome_probs(opts.iters, weight, heads);
        let p_value = 1.0 - run_within_prob;

        if p_value < alpha {
            println!("[{rand_type} trial {trial}] rejected H0: {heads} heads from {} runs at weight of {weight}, p={p_value} < {alpha}", opts.iters);
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

fn generate_weight_for_test(rng: &mut StdRng) -> f64 {
    let p = rng.next_u64() as f64 / u64::MAX as f64;
    assert!(p >= 0.0);
    assert!(p <= 1.0);
    p
}

/// Integrates the probabilities of all trial outcomes that are more likely than
/// that where `K=k`.
///
/// `n` — number of experiments in the sequence.
/// `w` — probability of success (equivalently, weight of the coin, where `w` > 0.5 is biased towards heads).
fn integrate_outcome_probs(n: u16, w: f64, k: u16) -> f64 {
    let outcome_prob = bernoulli_pmf(k, n, w);
    (0..=n)
        .map(|k| bernoulli_pmf(k, n, w))
        .filter(|&p| p > outcome_prob)
        .sum::<f64>()
        .min(1.0)
}

/// Obtains the Bernoulli Probability Mass Function.
///
/// `k` — number of success outcomes (equivalently, "heads").
/// `n` — number of experiments in the sequence.
/// `w` — probability of success (equivalently, weight of the coin, where `w` > 0.5 is biased towards heads).
fn bernoulli_pmf(k: u16, n: u16, w: f64) -> f64 {
    ncr(n, k) as f64 * w.powi(k as i32) * (1.0 - w).powi((n - k) as i32)
}

/// Calculates <sup>n</sup>C<sub>r</sub>.
fn ncr(n: u16, r: u16) -> u128 {
    assert!(n >= r);
    fact_trunc(n - r, n) / fact(r)
}

/// Calculates n!.
fn fact(n: u16) -> u128 {
    let mut fact = 1;
    for i in 2..=u128::from(n) {
        fact *= i;
    }
    fact
}

/// Calculates n!/(n-m)!.
fn fact_trunc(m: u16, n: u16) -> u128 {
    let mut fact = 1;
    for i in u128::from(m + 1)..=u128::from(n) {
        fact *= i;
    }
    fact
}

#[test]
fn self_test_bernoulli_pmf() {
    assert_float(0.059535, bernoulli_pmf(4, 6, 0.3));
}

fn assert_float(lhs: f64, rhs: f64) {
    assert!((rhs - lhs).abs() <= f64::EPSILON, "lhs={lhs} rhs={rhs}");
}

#[test]
fn self_test_fact() {
    assert_eq!(1, fact(1));
    assert_eq!(2, fact(2));
    assert_eq!(6, fact(3));
}

#[test]
fn self_test_fact_trunc() {
    assert_eq!(1, fact_trunc(1, 1));
    assert_eq!(2, fact_trunc(1, 2));
    assert_eq!(1, fact_trunc(2, 2));
    assert_eq!(6, fact_trunc(1, 3));
    assert_eq!(3, fact_trunc(2, 3));
    assert_eq!(1, fact_trunc(3, 3));
    assert_eq!(24, fact_trunc(1, 4));
    assert_eq!(12, fact_trunc(2, 4));
    assert_eq!(4, fact_trunc(3, 4));
}

#[test]
fn self_test_ncr() {
    assert_eq!(1, ncr(1, 1));
    assert_eq!(2, ncr(2, 1));
    assert_eq!(1, ncr(2, 2));
    assert_eq!(3, ncr(3, 1));
    assert_eq!(3, ncr(3, 2));
    assert_eq!(1, ncr(3, 3));
}
