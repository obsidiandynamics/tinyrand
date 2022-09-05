use alloc::format;
use crate::{Counter, Rand, Seeded};

#[test]
fn counter_from_zero() {
    let mut rand = Counter::default();
    assert_eq!(0, rand.next_u64());
    assert_eq!(1, rand.next_u64());
    assert_eq!(2, rand.next_u64());
    assert_eq!(3, rand.next_u64());
}

#[test]
fn counter_wrap() {
    let mut rand = Counter::seed(u64::MAX - 3);
    assert_eq!(u64::MAX - 3, rand.next_u64());
    assert_eq!(u64::MAX - 2, rand.next_u64());
    assert_eq!(u64::MAX - 1, rand.next_u64());
    assert_eq!(u64::MAX, rand.next_u64());
    assert_eq!(0, rand.next_u64());
    assert_eq!(1, rand.next_u64());
}

#[test]
fn implements_debug() {
    let rand = Counter::seed(42);
    let s = format!("{rand:?}");
    assert!(s.contains("Counter"));
    assert!(s.contains("42"));
}