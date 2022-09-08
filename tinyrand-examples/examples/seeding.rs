//! How to seed a [`Rand`].

use tinyrand::{Rand, Seeded, StdRand};
use tinyrand_std::{ClockSeed, thread_rand};

#[test]
fn run_main() {
    main();
}

fn main() {
    seed_from_u64();
    seed_from_clock();
    thread_local();
}

/// Seed from a user-specified number.
fn seed_from_u64() {
    let mut rand = StdRand::seed(42);
    let num = rand.next_u64();
    println!("generated {num}");
}

/// Seed from the system clock. Requires `tinyrand-std`.
fn seed_from_clock() {
    let seed = ClockSeed::default().next_u64();
    println!("seeding with {seed}");
    let mut rand = StdRand::seed(seed);
    let num = rand.next_u64();
    println!("generated {num}");
}

/// A thread-local RNG instance.
fn thread_local() {
    let mut rand = thread_rand();
    let num = rand.next_u64();
    println!("generated {num}");
}

