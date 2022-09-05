use crate::{cutoff_u128, Mock, Probability, Rand, RandRange, Wyrand, Xorshift};
use alloc::vec;
use core::ops::Range;
use crate::mock::{U64Cell, echo};

#[test]
fn random_range_wyrand() {
    __random_range(Wyrand::default());
}

#[test]
fn random_range_xorshift() {
    __random_range(Xorshift::default());
}

fn __random_range(mut rand: impl Rand) {
    #[derive(Debug)]
    struct Case {
        range: Range<u64>,
        exp_min: u64,
        exp_max: u64,
    }
    for case in &vec![
        // from zero
        Case {
            range: 0..0,
            exp_min: 0,
            exp_max: 0,
        },
        Case {
            range: 0..100,
            exp_min: 0,
            exp_max: 100 - 1,
        },
        Case {
            range: 0..1,
            exp_min: 0,
            exp_max: 0,
        },
        Case {
            range: 0..100,
            exp_min: 0,
            exp_max: 100 - 1,
        },
        Case {
            range: 0..100,
            exp_min: 0,
            exp_max: 100 - 1,
        },
        Case {
            range: 0..100,
            exp_min: 0,
            exp_max: 100 - 1,
        },
        Case {
            range: 0..u64::MAX,
            exp_min: 0,
            exp_max: u64::MAX - 1,
        },
        // from half
        Case {
            range: 50..100,
            exp_min: 50,
            exp_max: 100 - 1,
        },
        Case {
            range: 50..51,
            exp_min: 50,
            exp_max: 51 - 1,
        },
        Case {
            range: 50..100,
            exp_min: 50,
            exp_max: 100 - 1,
        },
        Case {
            range: 50..100,
            exp_min: 50,
            exp_max: 100 - 1,
        },
        Case {
            range: 50..100,
            exp_min: 50,
            exp_max: 100 - 1,
        },
        Case {
            range: u64::MAX >> 1..u64::MAX,
            exp_min: u64::MAX >> 1,
            exp_max: u64::MAX - 1,
        },
        // from top
        Case {
            range: 100..100,
            exp_min: 100,
            exp_max: 100,
        },
        Case {
            range: 100..100,
            exp_min: 100,
            exp_max: 100,
        },
        Case {
            range: 100..100,
            exp_min: 100,
            exp_max: 100,
        },
        Case {
            range: 100..100,
            exp_min: 100,
            exp_max: 100,
        },
        // excess
        Case {
            range: 101..100,
            exp_min: 101,
            exp_max: 101,
        },
    ] {
        let d = rand.next_range(case.range.clone());
        assert!(d >= case.exp_min, "for {case:?} random was {d:?}");
        assert!(d <= case.exp_max, "for {case:?} random was {d:?}");
    }
}

#[test]
#[should_panic(expected="cannot be less than 0")]
fn probability_panics_lt_0() {
    Probability::new(0f64 - f64::EPSILON);
}

#[test]
#[should_panic(expected="cannot be greater than 1")]
fn probability_panics_gt_1() {
    Probability::new(1f64 + f64::EPSILON);
}

#[test]
fn test_cutoff_u128() {
    assert_eq!(u128::MAX, cutoff_u128(1));
    assert_eq!(u128::MAX, cutoff_u128(2));
    assert_eq!(u128::MAX - 1, cutoff_u128(3));
}

#[test]
fn next_bool() {
    // NB: no matter what the random number, p(0.0) should always evaluate to false,
    // while p(1.0) should always evaluate to true

    let cell = U64Cell::default();
    let mut rand = Mock::new(echo(&cell));
    cell.set(0);
    assert!(!rand.next_bool(0.0.into()));
    assert!(rand.next_bool(f64::EPSILON.into()));
    assert!(rand.next_bool(0.5.into()));
    assert!(rand.next_bool(1.0.into()));

    cell.set(u64::MAX / 4);
    assert!(!rand.next_bool(0.0.into()));
    assert!(!rand.next_bool((0.25 - f64::EPSILON).into()));
    assert!(rand.next_bool((0.25 + f64::EPSILON).into()));
    assert!(rand.next_bool(1.0.into()));

    cell.set(u64::MAX / 2);
    assert!(!rand.next_bool(0.0.into()));
    assert!(!rand.next_bool((0.5 - f64::EPSILON).into()));
    assert!(rand.next_bool((0.5 + f64::EPSILON).into()));
    assert!(rand.next_bool(1.0.into()));

    cell.set(u64::MAX);
    assert!(!rand.next_bool(0.0.into()));
    assert!(!rand.next_bool(0.5.into()));
    assert!(!rand.next_bool((1.0 - f64::EPSILON).into()));
    assert!(rand.next_bool(1.0.into()));
}