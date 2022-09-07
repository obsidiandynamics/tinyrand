//! The bare essentials of using [`Rand`] and related traits.

use std::thread;
use std::time::Duration;
use tinyrand::{Probability, Rand, RandRange, StdRand};
use tinyrand_examples::SomeSpecialCondition;

#[test]
fn run_main() {
    main();
}

fn main() {
    next_u64();
    next_u128();
    next_lim();
    next_range();
    next_bool();
    random_sleep();
}

/// Generate a few `u64`s.
fn next_u64() {
    let mut rand = StdRand::default();
    for _ in 0..10 {
        let num = rand.next_u64();
        println!("generated {num}");
    }
}

/// Generate a `u128`.
fn next_u128() {
    let mut rand = StdRand::default();
    let num = rand.next_u128();
    println!("generated wider {num}");
}

/// Generate a number in the range [0, N).
fn next_lim() {
    const N: u64 = 42;
    let mut rand = StdRand::default();
    let num = rand.next_lim_u64(N);
    assert!(num < N);
    println!("generated {num}");
}

/// Generate a number in the given range.
fn next_range() {
    let mut rand = StdRand::default();
    let tasks = vec!["went to market", "stayed home", "had roast beef", "had none"];
    let random_index = rand.next_range(0..tasks.len());
    let random_task = tasks[random_index];
    println!("This little piggy {random_task}");
}

fn next_bool() {
    let mut rand = StdRand::default();
    let p = Probability::new(0.55); // a slightly weighted coin
    for _ in 0..10 {
        if rand.next_bool(p) {
            // expect to see more heads in the (sufficiently) long run
            println!("heads");
        } else {
            println!("tails");
        }
    }
}

fn random_sleep() {
    let mut rand = StdRand::default();
    let condition = SomeSpecialCondition::default();
    let base_sleep_micros = 10;
    let mut waits = 0;
    while !condition.has_happened() {
        let min_wait = Duration::ZERO;
        let max_wait = Duration::from_micros(base_sleep_micros * 2u64.pow(waits));
        let random_duration = rand.next_range(min_wait..max_wait);
        println!("backing off for {random_duration:?}");
        thread::sleep(random_duration);
        waits += 1;
    }
}

