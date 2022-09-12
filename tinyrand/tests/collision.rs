//! Conducts a series of trials on a [`Rand`] with a different (randomly chosen)
//! integer generation range on each trial. Within each trialled range, one random number
//! is chosen as the control value. A series of random numbers (sampled from the same range) is
//! then produced and the number of collisions with the control value is counted. By H0, the collisions
//! should follow a Poisson process with λ as the expected collision rate.

pub mod stats;

use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};
use std::ops::Range;
use tinyrand::{Counter, RandRange, Seeded, SplitMix, Wyrand, Xorshift};
use crate::stats::{holm_bonferroni_seq_correction, integrate_poisson, Rejection};

#[test]
fn collision_splitmix() {
    collision::<SplitMix>(Options::default()).unwrap();
}

#[test]
fn collision_wyrand() {
    collision::<Wyrand>(Options::default()).unwrap();
}

#[test]
fn collision_xorshift() {
    collision::<Xorshift>(Options::default()).unwrap();
}

#[test]
fn collision_counter_should_reject() {
    assert!(collision::<Counter>(Options::default()).is_err());
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

fn collision<S: Seeded>(opts: Options) -> Result<(), Vec<Rejection>>
where
    S::R: RandRange<u64>,
{
    opts.validate();
    let mut control_rng = StdRng::seed_from_u64(0);

    holm_bonferroni_seq_correction(opts.significance_level, opts.trials, || {
        let seed = control_rng.next_u64();
        let mut rand = S::seed(seed);
        let range = generate_range_for_test(&mut control_rng);
        let control = control_rng.gen_range(range.clone());

        let mut collisions = 0;
        for _ in 0..opts.iters {
            if rand.next_range(range.clone()) == control {
                collisions += 1;
            }
        }

        let width = range.end - range.start;
        let lambda = f64::from(opts.iters) / width as f64;
        let prob_integral = integrate_poisson(lambda, collisions);
        let p_value = 1.0 - prob_integral;
        //println!("width={width}, lambda={lambda}, collisions={collisions}, p_value={p_value}");
        p_value
    })
}

fn generate_range_for_test(rng: &mut StdRng) -> Range<u64> {
    const MIN_WIDTH: u64 = 64;
    const MAX_WIDTH: u64 = 128;

    let start = rng.gen_range(0..u64::MAX - MAX_WIDTH);
    let width = rng.gen_range(MIN_WIDTH..MAX_WIDTH);
    start..start + width
}
