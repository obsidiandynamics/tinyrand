use crate::{Rand, Seeded, Xorshift};

#[test]
fn create_default() {
    let mut rand = Xorshift::default();
    assert_ne!(0, rand.next_u64());
}

impl Xorshift {
    /// Constructs [`Xorshift`] directly, bypassing the usual zero-seed check. Don't try this
    /// at home, kids.
    fn construct(seed: u64) -> Self {
        Self(seed)
    }
}

#[test]
fn xorshift_constant_with_zero_seed() {
    // initialise with 0 seed for testing only; the user cannot initialise Xorshift with 0
    let mut rng = Xorshift::construct(0);
    assert_eq!(0, rng.next_u64());
    assert_eq!(0, rng.next_u64());
}

#[test]
fn create_seeded() {
    let mut rand = Xorshift::seed(0); // should not initialise from 0 seed under the hood
    assert_eq!(u64::MAX >> 1, rand.0);
    assert_ne!(0, rand.next_u64());

    let mut rand = Xorshift::seed(1); // every nonzero seed is okay
    assert_eq!(1, rand.0);
    assert_ne!(0, rand.next_u64());

    let mut rand = Xorshift::seed(u64::MAX); // every nonzero seed is okay
    assert_eq!(u64::MAX, rand.0);
    assert_ne!(0, rand.next_u64());
}