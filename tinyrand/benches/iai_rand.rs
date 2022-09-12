use iai::{main};
use tinyrand::{Probability, Rand, SplitMix, Wyrand, Xorshift};

fn splitmix_next_u64() -> u64 {
    let mut rand = SplitMix::default();
    rand.next_u64()
}

fn splitmix_next_bool() -> bool {
    let mut rand = SplitMix::default();
    rand.next_bool(Probability::new(0.5))
}

fn wyrand_next_u64() -> u64 {
    let mut rand = Wyrand::default();
    rand.next_u64()
}

fn wyrand_next_bool() -> bool {
    let mut rand = Wyrand::default();
    rand.next_bool(Probability::new(0.5))
}

fn xorshift_next_u64() -> u64 {
    let mut rand = Xorshift::default();
    rand.next_u64()
}

fn xorshift_next_bool() -> bool {
    let mut rand = Xorshift::default();
    rand.next_bool(Probability::new(0.5))
}

main!(splitmix_next_u64, splitmix_next_bool, wyrand_next_u64, wyrand_next_bool, xorshift_next_u64, xorshift_next_bool);