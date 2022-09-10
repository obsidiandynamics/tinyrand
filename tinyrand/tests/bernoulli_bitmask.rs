//! Conducts a series of Bernoulli trials on a [`Rand`] by masking the value of a single bit,
//! verifying that the number of times the bit is set to 1 is within the expected range. For
//! each subsequent trial, the mask is shifted one to the left and the experiment is repeated.
pub mod stats;

use crate::stats::{bonferroni_correction, integrate_bernoulli_outcome_probs, Options, Rejection};
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use tinyrand::{Counter, Rand, RandRange, Seeded, Wyrand, Xorshift};

#[test]
fn bitmask_wyrand() {
    bitmask_trial::<Wyrand>(bitmask_options()).unwrap();
}

#[test]
fn bitmask_xorshift() {
    bitmask_trial::<Xorshift>(bitmask_options()).unwrap();
}

#[test]
fn bitmask_counter_should_reject() {
    assert!(bitmask_trial::<Counter>(bitmask_options()).is_err());
}

#[test]
fn bitmask_broken_should_reject() {
    assert!(bitmask_trial::<LsbBrokenRand<Wyrand>>(bitmask_options()).is_err());
}

struct LsbBrokenRand<R: Rand>(R);

impl<R: Rand> Rand for LsbBrokenRand<R> {
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64() & 0xFFFF_FFFF_FFFF_FFFE
    }
}

impl<S: Seeded + Rand> Seeded for LsbBrokenRand<S> {
    type R = LsbBrokenRand<S::R>;

    fn seed(seed: u64) -> Self::R {
        LsbBrokenRand(S::seed(seed))
    }
}

fn bitmask_options() -> Options {
    Options {
        trials: 256,
        iters: 30,
        significance_level: 0.25,
    }
}

fn bitmask_trial<S: Seeded>(opts: Options) -> Result<(), Vec<Rejection>>
where
    S::R: RandRange<u64>,
{
    opts.validate();
    let mut control_rng = StdRng::seed_from_u64(0);

    let mut trial = 0;
    bonferroni_correction(opts.significance_level, opts.trials, || {
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
        let run_within_prob = integrate_bernoulli_outcome_probs(opts.iters, 0.5, set_bits);
        let p_value = 1.0 - run_within_prob;
        p_value
    })
}