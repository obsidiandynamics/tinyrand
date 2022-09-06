use alloc::rc::Rc;
use core::cell::RefCell;
use tinyrand::{Probability, Rand, RandLim};
use tinyrand::mock::{fixed, RefCellExt};
use crate::adv_mock::AdvMock;
use crate::echo_heap;

#[test]
fn next_bool() {
    let mut mock = AdvMock::default()
        .with_next_u64(fixed(0))
        .with_next_bool(|_, _| true);
    assert_eq!(0, mock.state.next_bool_invocations);
    assert!(mock.next_bool(Probability::new(0.0))); // absurd but true (because of mock)
    assert_eq!(1, mock.state.next_bool_invocations);
    assert!(mock.next_bool(Probability::new(0.5)));
    assert_eq!(2, mock.state.next_bool_invocations);
    assert!(mock.next_bool(Probability::new(1.0)));
    assert_eq!(3, mock.state.next_bool_invocations);

    let mut mock = AdvMock::default()
        .with_next_u64(fixed(0))
        .with_next_bool(|_, _| false);
    assert_eq!(0, mock.state.next_bool_invocations);
    assert!(!mock.next_bool(Probability::new(1.0))); // again, only possible thanks to mocking
    assert_eq!(1, mock.state.next_bool_invocations);
}

#[test]
fn next_bool_delegates_by_default() {
    let cell = Rc::new(RefCell::default());
    let mut mock = AdvMock::default()
        .with_next_u64(echo_heap(cell.clone()));
    assert_eq!(0, mock.state.next_bool_invocations);
    assert_eq!(0, mock.state.next_u64_invocations);
    assert!(!mock.next_bool(Probability::new(0.0)));
    assert!(mock.next_bool(Probability::new(0.5)));
    assert!(mock.next_bool(Probability::new(1.0)));
    assert_eq!(3, mock.state.next_bool_invocations);
    assert_eq!(3, mock.state.next_u64_invocations);

    cell.set(u64::MAX);
    assert!(!mock.next_bool(Probability::new(0.0)));
    assert!(!mock.next_bool(Probability::new(0.5)));
    assert!(mock.next_bool(Probability::new(1.0)));
    assert_eq!(6, mock.state.next_bool_invocations);
    assert_eq!(6, mock.state.next_u64_invocations);
}

#[test]
fn next_lim() {
    let mut mock = AdvMock::default()
        .with_next_u64(fixed(0))
        .with_next_lim_u128(|_, lim| lim / 2);
    assert_eq!(0, mock.state.next_lim_u128_invocations);
    assert_eq!(21, mock.next_lim_u16(42));
    assert_eq!(1, mock.state.next_lim_u128_invocations);
    assert_eq!(21, mock.next_lim_u32(42));
    assert_eq!(21, mock.next_lim_u64(42));
    assert_eq!(21, mock.next_lim_u128(42));
    assert_eq!(21, mock.next_lim_usize(42));
    assert_eq!(5, mock.state.next_lim_u128_invocations);
    assert_eq!(21, mock.next_lim(42u16));
    assert_eq!(21, mock.next_lim(42u32));
    assert_eq!(21, mock.next_lim(42u64));
    assert_eq!(21, mock.next_lim(42u128));
    assert_eq!(21, mock.next_lim(42usize));
    assert_eq!(10, mock.state.next_lim_u128_invocations);
}

#[test]
fn next_lim_delegates_by_default() {
    let cell = Rc::new(RefCell::default());
    let mut mock = AdvMock::default()
        .with_next_u64(echo_heap(cell.clone()));
    assert_eq!(0, mock.state.next_lim_u128_invocations);
    assert_eq!(0, mock.next_lim_u16(1));
    assert_eq!(1, mock.state.next_lim_u128_invocations);
    assert_eq!(0, mock.next_lim_u32(1));
    assert_eq!(0, mock.next_lim_u64(1));
    assert_eq!(0, mock.next_lim_u128(1));
    assert_eq!(0, mock.next_lim_usize(1));
    assert_eq!(5, mock.state.next_lim_u128_invocations);
    assert_eq!(0, mock.next_lim(1u16));
    assert_eq!(0, mock.next_lim(1u32));
    assert_eq!(0, mock.next_lim(1u64));
    assert_eq!(0, mock.next_lim(1u128));
    assert_eq!(0, mock.next_lim(1usize));
    assert_eq!(10, mock.state.next_lim_u128_invocations);
}