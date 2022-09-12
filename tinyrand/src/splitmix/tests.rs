use crate::{Rand, Seeded, SplitMix};
use crate::duration::tests::random_duration;
use crate::tests::{lim_types_max, next_types, numbers_differ, random_range_u128, random_range_u64, range_types_max};

#[test]
fn create_default() {
    let mut rand = SplitMix::default();
    assert_ne!(0, rand.next_u64());
}

impl SplitMix {
    /// Constructs [`SplitMix`] directly, bypassing the usual zero-seed check. Don't try this
    /// at home, kids.
    fn construct(seed: u64) -> Self {
        Self(seed)
    }
}

#[test]
fn splitmix_constant_with_zero_seed() {
    // initialise with 0 seed for testing only; the user cannot initialise SplitMix with 0
    let mut rng = SplitMix::construct(0);
    assert_eq!(0, rng.next_u64());
    assert_eq!(0, rng.next_u64());
}

#[test]
fn create_seeded() {
    let mut rand = SplitMix::seed(0); // should not initialise from 0 seed under the hood
    assert_eq!(u64::MAX >> 1, rand.0);
    assert_ne!(0, rand.next_u64());

    let mut rand = SplitMix::seed(1); // every nonzero seed is okay
    assert_eq!(1, rand.0);
    assert_ne!(0, rand.next_u64());

    let mut rand = SplitMix::seed(u64::MAX); // every nonzero seed is okay
    assert_eq!(u64::MAX, rand.0);
    assert_ne!(0, rand.next_u64());
}

#[test]
fn next_types_splitmix() {
    next_types(SplitMix::default());
}

#[test]
fn lim_types_max_splitmix() {
    lim_types_max(SplitMix::default());
}

#[test]
fn random_range_u64_splitmix() {
    random_range_u64(SplitMix::default());
}

#[test]
fn random_range_u128_splitmix() {
    random_range_u128(SplitMix::default());
}

#[test]
fn random_duration_splitmix() {
    random_duration(SplitMix::default());
}

#[test]
fn range_types_max_splitmix() { range_types_max(SplitMix::default()); }

#[test]
fn numbers_differ_splitmix() { numbers_differ(SplitMix::default()) }