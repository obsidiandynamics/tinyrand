# `tinyrand`
Lightweight RNG specification and several ultrafast implementations for Rust. `tinyrand` is `no_std`.

[![Crates.io](https://img.shields.io/crates/v/tinyrand?style=flat-square&logo=rust)](https://crates.io/crates/tinyrand)
[![docs.rs](https://img.shields.io/badge/docs.rs-tinyrand-blue?style=flat-square&logo=docs.rs)](https://docs.rs/tinyrand)
[![Build Status](https://img.shields.io/github/workflow/status/obsidiandynamics/tinyrand/Cargo%20build?style=flat-square&logo=github)](https://github.com/obsidiandynamics/tinyrand/actions/workflows/master.yml)
[![codecov](https://img.shields.io/codecov/c/github/obsidiandynamics/tinyrand/master?style=flat-square&logo=codecov)](https://codecov.io/gh/obsidiandynamics/tinyrand)
![no_std](https://img.shields.io/badge/linking-no__std-9cf?style=flat-square)

# Why `tinyrand`?
* It's very small and doesn't need `std`, meaning it's embeddable. It can run on microcontrollers and bare-metal (no OS) environments.
* It's very fast. Faster than others. It comes bundled with [Xorshift](https://en.wikipedia.org/wiki/Xorshift) and [Wyrand](https://github.com/wangyi-fudan/wyhash/blob/master/Modern%20Non-Cryptographic%20Hash%20Function%20and%20Pseudorandom%20Number%20Generator.pdf).
* The RNG behaviour is concisely specified as a handful of traits, independent of implementations. It makes it easy to swap implementations.
* It comes with [`Mock`](https://docs.rs/tinyrand/latest/tinyrand/mock/index.html) for testing code that depends on random numbers. That's if you care about code coverage.

# Performance
Below is a comparison of several notable RNGs.

| RNG        | Algorithm | Bandwidth (GB/s) |                                                                                       |
|:-----------|:----------|-----------------:|:--------------------------------------------------------------------------------------|
| `rand`     | ChaCha12  |              2.2 | <img src="https://via.placeholder.com/12/FF5733/FF5733.png" width="22" height="12"/>  |
| `fastrand` | Wyrand    |              5.1 | <img src="https://via.placeholder.com/12/FFC733/FFC733.png" width="51" height="12"/>  |
| `tinyrand` | Wyrand    |             14.6 | <img src="https://via.placeholder.com/12/33FFE0/33FFE0.png" width="146" height="12"/> |
| `tinyrand` | Xorshift  |              6.7 | <img src="https://via.placeholder.com/12/33FFE0/33FFE0.png" width="67" height="12"/>  |

TL;DR: `tinyrand` is almost 3x faster than `fastrand` and more than 6x faster than `rand`.

# Statistical properties
It's impossible to tell for certain whether a PRNG is good; the answer is probabilistic. `tinyrand` (both Wyrand and Xorshift) algorithms stand up well against the [Dieharder](http://webhome.phy.duke.edu/~rgb/General/dieharder.php) battery of tests. (Tested on 30.8 billion samples.) This means `tinyrand` produces numbers that appear sufficiently random and is likely fit for use in most applications.

`tinyrand` algorithms are not cryptographically secure, meaning it is possible to guess the next random number by observing a sequence of numbers. If you need a CSPRNG, it is strongly suggested that you go with `rand`. CSPRNGs are generally a lot slower and most folks don't need one.

# Getting started
## Add dependency
```sh
cargo add tinyrand
```

## The basics
A `Rand` instance is required to generate numbers. Here, we use `StdRand`, which is an alias for the default/recommended RNG. (Currently set to `Wyrand`, but may change in the future.)

```rust
let mut rand = StdRand::default();
for _ in 0..10 {
    let num = rand.next_u64();
    println!("generated {num}");
}
```

Similarly, we can generate numbers of other types:

```rust
let mut rand = StdRand::default();
let num = rand.next_u128();
println!("generated wider {num}");
```

The `next_uXX` methods generate numbers in the entire unsigned range of the specified type. Often, we want a number in a specific range:

```rust
let mut rand = StdRand::default();
let tasks = vec!["went to market", "stayed home", "had roast beef", "had none"];
let random_index = rand.next_range(0..tasks.len() as u64);
let random_task = tasks[random_index as usize];
println!("This little piggie {random_task}");
```

Another common use case is generating `bool`s. We might also want to assign a weighting to the binary outcomes:

```rust
let mut rand = StdRand::default();
let p = Probability::new(0.55); // a slightly weighted coin
for _ in 0..10 {
    if rand.next_bool(p) {
        println!("heads"); // expect to see more heads in the (sufficiently) long run
    } else {
        println!("tails");
    }
}
```

There are times when we need our thread to sleep for a while, waiting for a condition. When many threads are sleeping, it is generally recommended they back off randomly to avoiding a stampede.

```rust
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
```

## Seeding
Invoking `Default::default()` on a `Rand` initialises it with a constant seed. This is great for repeatability, but always results in the same run of "random" numbers, which is not what most folks need.

`tinyrand` is a `no_std` crate and, sadly, there is no good, portable way to generate entropy when one cannot make assumptions about the underlying platform. In most applications, one would use a clock, but something as simple as `SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)` mightn't be always available.

If you have an entropy source at your disposal, you could seed an `Rrnd` as so:

```rust
let seed = get_seed_from_somewhere();
let mut rand = StdRand::seed(seed);
let num = rand.next_u64();
println!("generated {num}");
```

If one doesn't care about `no_std`, they shouldn't be bound by its limitations. To seed from the system clock, you can opt in to `std`:

```
cargo add tinyrand-std
```

Now, we have a `ClockSeed` at our disposal, which also implements the `Rand` trait. `ClockSeed` derives a `u64` by XORing the upper 64 bits of the nanosecond timestamp (from `SystemTime`) with the lower 64 bits. It's not suitable for cryptographic use, but it will suit most general-purpose applications.

```rust
let seed = ClockSeed::default().next_u64();
println!("seeding with {seed}");
let mut rand = StdRand::seed(seed);
let num = rand.next_u64();
println!("generated {num}");
```