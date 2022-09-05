use iai::{main};
use tinyrand::{Probability, Rand, Wyrand, Xorshift};

fn wyrand_next_u64() -> u64 {
    let mut rand = Wyrand::default();
    rand.next_u64()
}

fn wyrand_gen_bool() -> bool {
    let mut rand = Wyrand::default();
    rand.next_bool(Probability::new(0.5))
}

fn xorshift_next_u64() -> u64 {
    let mut rand = Xorshift::default();
    rand.next_u64()
}

fn xorshift_gen_bool() -> bool {
    let mut rand = Xorshift::default();
    rand.next_bool(Probability::new(0.5))
}

main!(wyrand_next_u64, wyrand_gen_bool, xorshift_next_u64, xorshift_gen_bool);