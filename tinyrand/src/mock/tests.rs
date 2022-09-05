use alloc::format;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};
use crate::{Mock, Rand};
use crate::mock::{fixed, counter, echo, U64Cell};

#[test]
fn mock_counter() {
    let mut mock = Mock::new(counter(5..8));
    assert_eq!(0, mock.state.invocations);
    assert_eq!(5, mock.next_u64());
    assert_eq!(1, mock.state.invocations);
    assert_eq!(6, mock.next_u64());
    assert_eq!(2, mock.state.invocations);
    assert_eq!(7, mock.next_u64());
    assert_eq!(3, mock.state.invocations);
    assert_eq!(5, mock.next_u64());
    assert_eq!(4, mock.state.invocations);
}

#[test]
fn mock_fixed() {
    let mut mock = Mock::new(fixed(42));
    assert_eq!(0, mock.state.invocations);
    assert_eq!(42, mock.next_u64());
    assert_eq!(1, mock.state.invocations);
    assert_eq!(42, mock.next_u64());
}

#[test]
fn mock_echo() {
    let cell = U64Cell::default();
    let mut mock = Mock::new(echo(&cell));
    assert_eq!(0, mock.state.invocations);
    assert_eq!(0, mock.next_u64());
    cell.set(42);
    assert_eq!(1, mock.state.invocations);
    assert_eq!(42, mock.next_u64());
    assert_eq!(42, cell.get());
}

#[test]
fn invocations() {
    let invocations = Arc::new(AtomicU64::default());
    let mut mock = {
        let invocations = invocations.clone();
        Mock::new(move |state| {
            invocations.store(state.invocations(), Ordering::Relaxed);
            state.invocations * 100
        })
    };
    assert_eq!(0, mock.state.invocations());
    assert_eq!(0, invocations.load(Ordering::Relaxed));

    assert_eq!(0, mock.next_u64());
    assert_eq!(1, mock.state.invocations());
    assert_eq!(0, invocations.load(Ordering::Relaxed));

    assert_eq!(100, mock.next_u64());
    assert_eq!(2, mock.state.invocations());
    assert_eq!(1, invocations.load(Ordering::Relaxed));
}

#[test]
fn u64cell_implements_debug() {
    let d = format!("{:?}", U64Cell::new(42));
    assert!(d.contains("U64Cell"));
    assert!(d.contains("42"));
}