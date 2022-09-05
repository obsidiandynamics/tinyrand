use crate::{cutoff_u128, Mock, Probability, Rand, RandLim, RandRange, Wyrand, Xorshift};
use alloc::{format, vec};
use core::ops::Range;
use crate::mock::{U64Cell, echo, counter, fixed};

#[test]
fn gen_128_bit_from_64() {
    let mut mock = Mock::new(counter(1..3));
    let next = mock.next_u128();
    assert_eq!(0x0000_0000_0000_0001__0000_0000_0000_0002, next);
}

#[test]
fn lim_64() {
    let mut mock = Mock::new(counter(u64::MAX >> 1..u64::MAX));
    for lim in (u64::MAX >> 1)..(u64::MAX >> 1) + 100 {
        let _ = mock.next_lim(lim);
    }
}

#[test]
fn lim_128_small() {
    let mut mock = Mock::new(counter(u64::MAX -17..u64::MAX));
    for lim in 1..13u128 {
        let _ = mock.next_lim(lim);
    }
}

#[test]
fn lim_128_large() {
    let mut mock = Mock::new(counter(u64::MAX -17..u64::MAX));
    for lim in u64::MAX as u128..u64::MAX as u128 + 13u128 {
        let _ = mock.next_lim(lim);
    }
}

#[test]
#[should_panic(expected="zero limit")]
fn zero_lim_64() {
    Mock::new(fixed(0)).next_lim(0u64);
}

#[test]
#[should_panic(expected="zero limit")]
fn zero_lim_128() {
    Mock::new(fixed(0)).next_lim(0u128);
}

#[test]
#[should_panic(expected="empty range")]
fn empty_u64_range() {
    let mut rand = Mock::new(fixed(0));
    rand.next_range(0..0u64);
}

#[test]
#[should_panic(expected="empty range")]
fn empty_u128_range() {
    let mut rand = Mock::new(fixed(0));
    rand.next_range(0..0u128);
}

#[test]
fn random_range_u64_wyrand() {
    __random_range_u64(Wyrand::default());
}

#[test]
fn random_range_u64_xorshift() {
    __random_range_u64(Xorshift::default());
}

fn __random_range_u64(mut rand: impl Rand) {
    #[derive(Debug)]
    struct Case {
        range: Range<u64>,
        exp_min: u64,
        exp_max: u64,
    }
    for case in &vec![
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
            range: 0..u64::MAX,
            exp_min: 0,
            exp_max: u64::MAX - 1,
        },
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
            range: u64::MAX >> 1..u64::MAX,
            exp_min: u64::MAX >> 1,
            exp_max: u64::MAX - 1,
        },
    ] {
        let d = rand.next_range(case.range.clone());
        assert!(d >= case.exp_min, "for {case:?} random was {d:?}");
        assert!(d <= case.exp_max, "for {case:?} random was {d:?}");
    }
}

#[test]
fn random_range_u128_wyrand() {
    __random_range_u128(Wyrand::default());
}

#[test]
fn random_range_u128_xorshift() {
    __random_range_u128(Xorshift::default());
}

fn __random_range_u128(mut rand: impl Rand) {
    #[derive(Debug)]
    struct Case {
        range: Range<u128>,
        exp_min: u128,
        exp_max: u128,
    }
    for case in &vec![
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
            range: 0..u128::MAX,
            exp_min: 0,
            exp_max: u128::MAX - 1,
        },
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
            range: u128::MAX >> 1..u128::MAX,
            exp_min: u128::MAX >> 1,
            exp_max: u128::MAX - 1,
        },
    ] {
        let d = rand.next_range(case.range.clone());
        assert!(d >= case.exp_min, "for {case:?} random was {d:?}");
        assert!(d <= case.exp_max, "for {case:?} random was {d:?}");
    }
}

#[test]
fn probability_within_valid_range() {
    assert_eq!(0.0, Probability::from(0.0).into());
    assert_eq!(0.0 + f64::EPSILON, Probability::from(0.0 + f64::EPSILON).into());
    assert_eq!(1.0 - f64::EPSILON, Probability::from(1.0 - f64::EPSILON).into());
    assert_eq!(1.0, Probability::from(1.0).into());
}

#[test]
#[should_panic(expected="cannot be less than 0")]
fn probability_panics_lt_0() {
    Probability::new(0.0 - f64::EPSILON);
}

#[test]
#[should_panic(expected="cannot be greater than 1")]
fn probability_panics_gt_1() {
    Probability::new(1.0 + f64::EPSILON);
}

#[test]
fn probability_unchecked() {
    // One can create probabilities outside the range [0, 1], but it's a sin to do so.
    let p = 0.0 - f64::EPSILON;
    unsafe {
        assert_eq!(p, Probability::new_unchecked(p).into());
    }

    let p = 1.0 + f64::EPSILON;
    unsafe {
        assert_eq!(p, Probability::new_unchecked(p).into());
    }
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

#[test]
fn probability_implements_copy() {
    let p = Probability::new(0.75);
    let p1 = p;
    let p2 = p;
    assert_eq!(p, p1);
    assert_eq!(p1, p2);
}

#[test]
fn probability_implements_debug() {
    let d = format!("{:?}", Probability::new(0.75));
    assert!(d.contains("Probability"));
    assert!(d.contains("0.75"));
}