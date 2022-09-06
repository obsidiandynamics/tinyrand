use alloc::rc::Rc;
use tinyrand::{Probability, Rand};
use tinyrand::mock::{fixed, U64Cell};
use crate::adv_mock::AdvMock;
use crate::echo_heap;

#[test]
fn next_bool() {
    let mut mock = AdvMock::default()
        .with_next_u64(fixed(0))
        .with_next_bool(|_, _| true);
    assert_eq!(0, mock.state.next_bool_invocations);
    assert!(mock.next_bool(Probability::new(0.0))); // absurd but true (because mock)
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
    let cell = Rc::new(U64Cell::default());
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