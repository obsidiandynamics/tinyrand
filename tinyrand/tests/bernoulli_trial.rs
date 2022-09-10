//! Conducts a series of Bernoulli trials on a [`Rand`] with a different (randomly chosen)
//! weighting on each trial, simulating a run of coin flips. Within each trial,
//! H0 asserts that the source is random. (I.e.,
//! the number of 'heads' falls within a statistically acceptable interval.)
//! Multiple Bernoulli trials are run using Bonferroni correction to depress
//! the Type I error rate. I.e., even an ideal random source, subjected to sufficient number
//! of trials, will fail some of them. The significance level is, therefore, scaled to minimise
//! false rejections.

pub mod stats;

use crate::stats::{bonferroni_correction, integrate_bernoulli_outcome_probs, Options, Rejection};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use tinyrand::{Counter, Probability, Rand, RandRange, Seeded, Wyrand, Xorshift};

#[test]
fn bernoulli_trial_wyrand() {
    coin_flip::<Wyrand>(bernoulli_trial_options()).unwrap();
}

#[test]
fn bernoulli_trial_xorshift() {
    coin_flip::<Xorshift>(bernoulli_trial_options()).unwrap();
}

#[test]
fn bernoulli_trial_counter_should_reject() {
    assert!(coin_flip::<Counter>(bernoulli_trial_options()).is_err());
}

fn bernoulli_trial_options() -> Options {
    Options {
        trials: 100,
        iters: 30,
        significance_level: 0.25,
    }
}

/// Runs a series of Bernoulli trials using Bonferroni correction to depress the Type I error rate.
fn coin_flip<S: Seeded>(opts: Options) -> Result<(), Vec<Rejection>>
where
    S::R: RandRange<u64>,
{
    opts.validate();
    let mut control_rng = StdRng::seed_from_u64(0);

    bonferroni_correction(opts.significance_level, opts.trials, || {
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
        let run_within_prob = integrate_bernoulli_outcome_probs(opts.iters, weight, heads);
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
