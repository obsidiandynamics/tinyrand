use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};
use crate::{Mock, Rand};
use crate::mock::{constant, counter};

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
fn mock_constant() {
    let mut mock = Mock::new(constant(42));
    assert_eq!(0, mock.state.invocations);
    assert_eq!(42, mock.next_u64());
    assert_eq!(1, mock.state.invocations);
    assert_eq!(42, mock.next_u64());
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