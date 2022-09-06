use alloc::sync::Arc;
use core::cell::RefCell;
use core::sync::atomic::{AtomicU64, Ordering};
use crate::{Rand};
use crate::mock::{fixed, counter, echo, Mock, RefCellExt, Next};

#[test]
fn implements_next() {
    assert_eq!(7, 6u64.next());
    assert_eq!(7, 6u128.next());
}

#[test]
fn mock_counter() {
    let mut mock = Mock::new(counter(5..8));
    assert_eq!(0, mock.state().next_u64_invocations());
    assert_eq!(5, mock.next_u64());
    assert_eq!(1, mock.state().next_u64_invocations());
    assert_eq!(6, mock.next_u64());
    assert_eq!(2, mock.state().next_u64_invocations());
    assert_eq!(7, mock.next_u64());
    assert_eq!(3, mock.state().next_u64_invocations());
    assert_eq!(5, mock.next_u64());
    assert_eq!(4, mock.state().next_u64_invocations());
}

#[test]
fn mock_fixed() {
    let mut mock = Mock::new(fixed(42));
    assert_eq!(0, mock.state().next_u64_invocations());
    assert_eq!(42, mock.next_u64());
    assert_eq!(1, mock.state().next_u64_invocations());
    assert_eq!(42, mock.next_u64());
}

#[test]
fn mock_echo() {
    let cell = RefCell::default();
    let mut mock = Mock::new(echo(&cell));
    assert_eq!(0, mock.state().next_u64_invocations());
    assert_eq!(0, mock.next_u64());
    cell.set(42);
    assert_eq!(1, mock.state().next_u64_invocations());
    assert_eq!(42, mock.next_u64());
    assert_eq!(42, cell.get());
}

#[test]
fn invocations() {
    let invocations = Arc::new(AtomicU64::default());
    let mut mock = {
        let invocations = invocations.clone();
        Mock::new(move |state| {
            invocations.store(state.next_u64_invocations(), Ordering::Relaxed);
            state.next_u64_invocations() * 100
        })
    };
    assert_eq!(0, mock.state().next_u64_invocations());
    assert_eq!(0, invocations.load(Ordering::Relaxed));

    assert_eq!(0, mock.next_u64());
    assert_eq!(1, mock.state().next_u64_invocations());
    assert_eq!(0, invocations.load(Ordering::Relaxed));

    assert_eq!(100, mock.next_u64());
    assert_eq!(2, mock.state().next_u64_invocations());
    assert_eq!(1, invocations.load(Ordering::Relaxed));
}