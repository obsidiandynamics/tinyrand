use crate::{Rand, Seeded, Wyrand};
use crate::duration::tests::random_duration;
use crate::tests::{lim_types_max, next_types, numbers_differ, random_range_u128, random_range_u64, range_types_max};

#[test]
fn create_default() {
    let mut rand = Wyrand::default();
    assert_ne!(0, rand.next_u64());
}

#[test]
fn create_seeded() {
    let mut rand = Wyrand::seed(0);
    assert_eq!(0, rand.0);
    assert_ne!(0, rand.next_u64());

    let mut rand = Wyrand::seed(1);
    assert_eq!(1, rand.0);
    assert_ne!(0, rand.next_u64());

    let mut rand = Wyrand::seed(u64::MAX);
    assert_eq!(u64::MAX, rand.0);
    assert_ne!(0, rand.next_u64());
}

#[test]
fn next_types_wyrand() {
    next_types(Wyrand::default());
}

#[test]
fn lim_types_max_wyrand() {
    lim_types_max(Wyrand::default());
}

#[test]
fn random_range_u64_wyrand() {
    random_range_u64(Wyrand::default());
}

#[test]
fn random_range_u128_wyrand() {
    random_range_u128(Wyrand::default());
}

#[test]
fn random_duration_wyrand() {
    random_duration(Wyrand::default());
}

#[test]
fn range_types_max_wyrand() { range_types_max(Wyrand::default()); }

#[test]
fn numbers_differ_wyrand() { numbers_differ(Wyrand::default()) }