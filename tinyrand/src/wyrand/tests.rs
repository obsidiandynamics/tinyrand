use crate::{Rand, Seeded, Wyrand};

#[test]
fn create_default() {
    let mut rand = Wyrand::default();
    assert_ne!(0, rand.next_u64());
}

#[test]
fn create_seeded() {
    let mut rand = Wyrand::seed(0);
    assert_eq!(0, rand.0);
    assert_ne!(0, rand.next_u64());

    let mut rand = Wyrand::seed(1);
    assert_eq!(1, rand.0);
    assert_ne!(0, rand.next_u64());

    let mut rand = Wyrand::seed(u64::MAX);
    assert_eq!(u64::MAX, rand.0);
    assert_ne!(0, rand.next_u64());
}