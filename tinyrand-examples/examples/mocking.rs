//! Mocking [`Rand`] behaviour.

use std::cell::RefCell;
use tinyrand::{Probability, Rand, RandRange, RefCellExt};
use tinyrand_alloc::{counter, echo, fixed, Mock};

#[test]
fn run_main() {
    main();
}

fn main() {
    with_basic_closure();
    with_fancy_closure();
    with_value_from_state();
    with_fixed_value();
    with_changing_delegate();
    with_counter();
    with_echo();
    with_bool();
    with_bool_from_state();
    with_lim();
    with_range();
}

fn with_basic_closure() {
    let mut rand = Mock::default().with_next_u128(|_| 42);
    for _ in 0..10 {
        assert_eq!(42, rand.next_usize()); // always 42
    }
    assert_eq!(10, rand.state().next_u128_invocations());
}

fn with_fancy_closure() {
    let val = RefCell::new(3);
    let mut rand = Mock::default().with_next_u128(|_| *val.borrow());

    assert_eq!(3, rand.next_usize());

    // ... later ...
    *val.borrow_mut() = 17;
    assert_eq!(17, rand.next_usize());
}

fn with_value_from_state() {
    let mut rand = Mock::default().with_next_u128(|state| {
        // return number of completed invocations
        state.next_u128_invocations() as u128
    });
    assert_eq!(0, rand.next_usize());
    assert_eq!(1, rand.next_usize());
    assert_eq!(2, rand.next_usize());
}

fn with_fixed_value() {
    let mut rand = Mock::default().with_next_u128(fixed(42));
    for _ in 0..10 {
        assert_eq!(42, rand.next_usize()); // always 42
    }
}

fn with_changing_delegate() {
    let mut rand = Mock::default().with_next_u128(fixed(42));
    assert_eq!(42, rand.next_usize());

    rand = rand.with_next_u128(fixed(88));
    assert_eq!(88, rand.next_usize());
}

fn with_counter() {
    let mut rand = Mock::default().with_next_u128(counter(5..8));
    assert_eq!(5, rand.next_usize());
    assert_eq!(6, rand.next_usize());
    assert_eq!(7, rand.next_usize());
    assert_eq!(5, rand.next_usize()); // start again
}

fn with_echo() {
    let cell = RefCell::new(42);
    let mut rand = Mock::default().with_next_u128(echo(&cell));
    assert_eq!(42, rand.next_usize());
    cell.set(66);
    assert_eq!(66, rand.next_usize());
}

fn with_bool() {
    let mut rand = Mock::default().with_next_bool(|_, _| false);
    if rand.next_bool(Probability::new(0.999999)) {
        println!("very likely");
    } else {
        // we can cover this branch thanks to the magic of mocking
        println!("very unlikely");
    }
}

fn with_bool_from_state() {
    let mut rand = Mock::default().with_next_bool(|surrogate, _| {
        surrogate.state().next_bool_invocations() % 2 == 0
    });
    assert_eq!(true, rand.next_bool(Probability::new(0.5)));
    assert_eq!(false, rand.next_bool(Probability::new(0.5)));
    assert_eq!(true, rand.next_bool(Probability::new(0.5)));
    assert_eq!(false, rand.next_bool(Probability::new(0.5)));
}

fn with_lim() {
    enum Day {
        Mon, Tue, Wed, Thu, Fri, Sat, Sun
    }
    const DAYS: [Day; 7] = [Day::Mon, Day::Tue, Day::Wed, Day::Thu, Day::Fri, Day::Sat, Day::Sun];

    let mut rand = Mock::default().with_next_lim_u128(|_, _| 6);
    let day = &DAYS[rand.next_range(0..DAYS.len())];
    assert!(matches!(day, Day::Sun)); // always a Sunday
    assert!(matches!(day, Day::Sun));
}

fn with_range() {
    let mut rand = Mock::default().with_next_lim_u128(|_, _| 6);
    assert_eq!(1006, rand.next_range(1000..2000u32));
}