use tinyrand::{Probability, Rand};
use crate::adv_mock::{AdvMock, fixed};

#[test]
fn next_bool() {
    let mut mock = AdvMock::new(fixed(0))
        .with_next_bool(|_, _, _| true);
    assert_eq!(0, mock.state.next_bool_invocations);
    assert!(mock.next_bool(Probability::new(0.0))); // absurd but true (because mock)
    assert_eq!(1, mock.state.next_bool_invocations);
    assert!(mock.next_bool(Probability::new(0.5)));
    assert_eq!(2, mock.state.next_bool_invocations);
    assert!(mock.next_bool(Probability::new(1.0)));
    assert_eq!(3, mock.state.next_bool_invocations);

    let mut mock = AdvMock::new(fixed(0))
        .with_next_bool(|_, _, _| false);
    assert_eq!(0, mock.state.next_bool_invocations);
    assert!(!mock.next_bool(Probability::new(1.0))); // again, only possible thanks to mocking
    assert_eq!(1, mock.state.next_bool_invocations);
}