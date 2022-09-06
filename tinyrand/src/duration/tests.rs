use crate::duration::from_nanos;
use crate::{Mock, Rand, RandRange};
use alloc::vec;
use core::ops::Range;
use core::time::Duration;
use crate::mock::fixed;

#[test]
fn duration_from_nanos_reversible() {
    let cases = vec![
        Duration::ZERO,
        Duration::from_nanos(1),
        Duration::from_micros(1),
        Duration::from_millis(1),
        Duration::from_secs(1),
        Duration::MAX,
    ];

    for case in cases {
        let nanos = case.as_nanos();
        let duration = from_nanos(nanos);
        assert_eq!(case, duration);
    }
}

pub fn random_duration(mut rand: impl Rand) {
    const NANOSECOND: Duration = Duration::new(0, 1);

    #[derive(Debug)]
    struct Case {
        range: Range<Duration>,
        exp_min: Duration,
        exp_max: Duration,
    }
    for case in &vec![
        // from zero
        Case {
            range: Duration::ZERO..Duration::from_nanos(100),
            exp_min: Duration::ZERO,
            exp_max: Duration::from_nanos(100) - NANOSECOND,
        },
        Case {
            range: Duration::ZERO..Duration::from_nanos(1),
            exp_min: Duration::ZERO,
            exp_max: Duration::ZERO,
        },
        Case {
            range: Duration::ZERO..Duration::from_micros(100),
            exp_min: Duration::ZERO,
            exp_max: Duration::from_micros(100) - NANOSECOND,
        },
        Case {
            range: Duration::ZERO..Duration::from_millis(100),
            exp_min: Duration::ZERO,
            exp_max: Duration::from_millis(100) - NANOSECOND,
        },
        Case {
            range: Duration::ZERO..Duration::from_secs(100),
            exp_min: Duration::ZERO,
            exp_max: Duration::from_secs(100) - NANOSECOND,
        },
        Case {
            range: Duration::ZERO..Duration::MAX,
            exp_min: Duration::ZERO,
            exp_max: Duration::MAX - NANOSECOND,
        },
        // from half
        Case {
            range: Duration::from_nanos(50)..Duration::from_nanos(100),
            exp_min: Duration::from_nanos(50),
            exp_max: Duration::from_nanos(100) - NANOSECOND,
        },
        Case {
            range: Duration::from_nanos(50)..Duration::from_nanos(51),
            exp_min: Duration::from_nanos(50),
            exp_max: Duration::from_nanos(51) - NANOSECOND,
        },
        Case {
            range: Duration::from_micros(50)..Duration::from_micros(100),
            exp_min: Duration::from_micros(50),
            exp_max: Duration::from_micros(100) - NANOSECOND,
        },
        Case {
            range: Duration::from_millis(50)..Duration::from_millis(100),
            exp_min: Duration::from_millis(50),
            exp_max: Duration::from_millis(100) - NANOSECOND,
        },
        Case {
            range: Duration::from_secs(50)..Duration::from_secs(100),
            exp_min: Duration::from_secs(50),
            exp_max: Duration::from_secs(100) - NANOSECOND,
        },
        Case {
            range: Duration::from_secs(u64::MAX >> 1)..Duration::MAX,
            exp_min: Duration::from_secs(u64::MAX >> 1),
            exp_max: Duration::MAX - NANOSECOND,
        },
    ] {
        let d = rand.next_range(case.range.clone());
        assert!(d >= case.exp_min, "for {case:?} random duration was {d:?}");
        assert!(d <= case.exp_max, "for {case:?} random duration was {d:?}");
    }
}

#[test]
#[should_panic(expected="empty range")]
fn empty_duration_range() {
    let mut rand = Mock::new(fixed(0));
    rand.next_range(Duration::ZERO..Duration::ZERO);
}