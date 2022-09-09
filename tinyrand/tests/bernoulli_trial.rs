use std::any;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use tinyrand::{Counter, Probability, Rand, RandRange, Seeded, Wyrand, Xorshift};

#[test]
fn bernoulli_trial_wyrand() {
    bernoulli_trial::<Wyrand>(Options::default());
}

#[test]
fn bernoulli_trial_xorshift() {
    bernoulli_trial::<Xorshift>(Options::default());
}

#[test]
#[should_panic(expected="rejected H0")]
fn bernoulli_trial_counter() {
    bernoulli_trial::<Counter>(Options::default());
}

#[derive(Debug)]
struct Options {
    cycles: u32,
    iters: u32,
    confidence_level: f64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            cycles: 100,
            iters: 10,
            confidence_level: 0.95,
        }
    }
}

impl Options {
    fn validate(&self) {
        assert!(self.cycles > 0);
        assert!(self.iters > 0);
        assert!(self.confidence_level >= f64::EPSILON);
        assert!(self.confidence_level <= 1.0 - f64::EPSILON);
    }
}

fn bernoulli_trial<S: Seeded>(opts: Options)
where
    S::R: RandRange<u64>,
{
    opts.validate();
    let rand_type = any::type_name::<S>();
    let mut control_rng = StdRng::seed_from_u64(0);
    let mut rejections = 0;

    for cycle in 0..opts.cycles {
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
        let run_within_prob = integrate_outcome_probs(opts.iters, weight,heads);
        let p_value = 1.0 - run_within_prob;

        if 1.0 - opts.confidence_level > p_value {
            println!("[{rand_type} cycle {cycle}] rejected H0: {heads} heads from {} runs at weight of {weight}, p={p_value}", opts.iters);
            rejections += 1;
        }
    }

    let p_value = f64::from(rejections) / f64::from(opts.cycles);
    println!("[{rand_type}] rejections/cycles: {rejections}/{}, p={p_value}", opts.cycles);
    assert!(p_value < 1.0 - opts.confidence_level, "rejected H0: rejections/cycles: {rejections}/{}, p={p_value}", opts.cycles);
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
fn integrate_outcome_probs(n: u32, w: f64, k: u32) -> f64{
    let outcome_prob = bernoulli_pmf(k, n, w);
    (0..=n)
        .map(|k| bernoulli_pmf(k, n, w))
        .filter(|&p| p > outcome_prob)
        .sum()
}

/// Obtains the Bernoulli Probability Mass Function.
///
/// `k` — number of success outcomes (equivalently, "heads").
/// `n` — number of experiments in the sequence.
/// `w` — probability of success (equivalently, weight of the coin, where `w` > 0.5 is biased towards heads).
fn bernoulli_pmf(k: u32, n: u32, w: f64) -> f64 {
    ncr(n, k) as f64 * w.powi(k as i32) * (1.0 - w).powi((n - k) as i32)
}

/// Calculates <sup>n</sup>C<sub>r</sub>.
fn ncr(n: u32, r: u32) -> u128 {
    assert!(n >= r);
    fact_trunc(n - r, n) / fact(r)
}

/// Calculates n!.
fn fact(n: u32) -> u128 {
    let mut fact = 1;
    for i in 2..=u128::from(n) {
        fact *= i;
    }
    fact
}

/// Calculates n!/(n-m)!.
fn fact_trunc(m: u32, n: u32) -> u128 {
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
