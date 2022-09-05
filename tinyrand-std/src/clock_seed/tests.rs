use tinyrand::Rand;
use crate::ClockSeed;

#[test]
fn clock_seed() {
    let mut seed = ClockSeed::default();
    assert_ne!(0, seed.next_u64());
}