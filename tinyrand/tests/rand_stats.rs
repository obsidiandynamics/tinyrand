use std::any;
use rand::rngs::{StdRng};
use rand::{RngCore, SeedableRng};
use tinyrand::{Rand, Seeded, Wyrand, Xorshift};

#[test]
fn mean_convergence_xorshift() {
    __mean_convergence::<Xorshift>(Options::default());
}

#[test]
fn mean_convergence_wyrand() {
    __mean_convergence::<Wyrand>(Options::default());
}

#[derive(Debug)]
struct Options {
    cycles: usize,
    min_iters: usize,
    max_iters: usize,
    tolerance: f64
}

impl Default for Options {
    fn default() -> Self {
        Self {
            cycles: 100,
            min_iters: 100,
            max_iters: 10_000_000,
            tolerance: 0.001
        }
    }
}

impl Options {
    fn validate(&self) {
        assert!(self.cycles > 0);
        assert!(self.min_iters <= self.max_iters);
        assert!(self.tolerance >= f64::EPSILON);
    }
}

fn __mean_convergence<S: Seeded>(opts: Options) {
    opts.validate();

    let allowed_width: u64 = (u64::MAX as f64 * opts.tolerance / 2.0) as u64;
    let expectation: u64 = u64::MAX >> 1;
    let expectation_min: u64 = expectation - allowed_width;
    let expectation_max: u64 = expectation + allowed_width;

    let mut control_rand = StdRng::seed_from_u64(0);

    for cycle in 0..opts.cycles {
        let seed = control_rand.next_u64();
        let mut rand = S::seed(seed);
        let mut sum = 0u128;
        for iter in 1..=opts.max_iters {
            sum += rand.next_u64() as u128;
            if iter >= opts.min_iters {
                let mean = (sum as f64 / iter as f64) as u64;
                // println!("iter={iter}, avg={avg}");
                if mean < expectation_min || mean > expectation_max {
                    if iter >= opts.max_iters {
                        assert!(mean >= expectation_min, "{mean} < {expectation_min} after {iter} iterations for seed {seed} [cycle {cycle}, {opts:?}, {}]", any::type_name::<S>());
                        assert!(mean <= expectation_max, "{mean} > {expectation_max} after {iter} iterations for seed {seed} [cycle {cycle}, {opts:?}, {}]", any::type_name::<S>());
                    }
                } else {
                    break;
                }
            }
        }
    }
}