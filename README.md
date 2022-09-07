# `tinyrand`
Lightweight RNG specification and several ultrafast implementations in Rust. `tinyrand` is `no_std` and doesn't use a heap allocator.

[![Crates.io](https://img.shields.io/crates/v/tinyrand?style=flat-square&logo=rust)](https://crates.io/crates/tinyrand)
[![docs.rs](https://img.shields.io/badge/docs.rs-tinyrand-blue?style=flat-square&logo=docs.rs)](https://docs.rs/tinyrand)
[![Build Status](https://img.shields.io/github/workflow/status/obsidiandynamics/tinyrand/Cargo%20build?style=flat-square&logo=github)](https://github.com/obsidiandynamics/tinyrand/actions/workflows/master.yml)
[![codecov](https://img.shields.io/codecov/c/github/obsidiandynamics/tinyrand/master?style=flat-square&logo=codecov)](https://codecov.io/gh/obsidiandynamics/tinyrand)
![no_std](https://img.shields.io/badge/linking-no__std-9cf?style=flat-square)

# Why `tinyrand`?
* It's very small and doesn't need `std`, meaning it's embeddable — it runs on microcontrollers and bare-metal (no OS) environments.
* It's very fast. It comes bundled with [Xorshift](https://en.wikipedia.org/wiki/Xorshift) and [Wyrand](https://github.com/wangyi-fudan/wyhash/blob/master/Modern%20Non-Cryptographic%20Hash%20Function%20and%20Pseudorandom%20Number%20Generator.pdf).
* The RNG behaviour is concisely specified as a handful of traits, independent of the underlying implementations. It makes it easy to swap implementations.
* It comes with [`Mock`](https://docs.rs/tinyrand-alloc/latest/tinyrand-alloc/mock/index.html) for testing code that depends on random numbers. That is, if you care about code coverage.

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
It's impossible to tell for certain whether a certain PRNG is good; the answer is probabilistic. Both the Wyrand and Xorshift algorithms stand up well against the [Dieharder](http://webhome.phy.duke.edu/~rgb/General/dieharder.php) barrage of tests. (Tested on 30.8 billion samples.) This means `tinyrand` produces numbers that appear sufficiently random and is likely fit for use in most applications.

`tinyrand` algorithms are not cryptographically secure, meaning it is possible to guess the next random number by observing a sequence of numbers. (Or the preceding numbers, for that matter.) If you need a robust CSPRNG, it is strongly suggested that you go with `rand`. CSPRNGs are generally a lot slower and most folks don't need one.

# Getting started
## Add dependency
```sh
cargo add tinyrand
```

## The basics
A `Rand` instance is required to generate numbers. Here, we use `StdRand`, which is an alias for the default/recommended RNG. (Currently set to `Wyrand`, but may change in the future.)

```rust
use tinyrand::{Rand, StdRand};

let mut rand = StdRand::default();
for _ in 0..10 {
    let num = rand.next_u64();
    println!("generated {num}");
}
```

Similarly, we can generate numbers of other types:

```rust
use tinyrand::{Rand, StdRand};

let mut rand = StdRand::default();
let num = rand.next_u128();
println!("generated wider {num}");
```

The `next_uXX` methods generate numbers in the entire unsigned range of the specified type. Often, we want a number in a specific range:

```rust
use tinyrand::{Rand, StdRand, RandRange};

let mut rand = StdRand::default();
let tasks = vec!["went to market", "stayed home", "had roast beef", "had none"];
let random_index = rand.next_range(0..tasks.len());
let random_task = tasks[random_index];
println!("This little piggy {random_task}");
```

Another common use case is generating `bool`s. We might also want to assign a weighting to the binary outcomes:

```rust
use tinyrand::{Rand, StdRand, Probability};

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
```

There are times when we need our thread to sleep for a while, waiting for a condition. When many threads are sleeping, it is generally recommended they back off randomly to avoid a stampede.

```rust
use tinyrand::{Rand, StdRand, RandRange};
use core::time::Duration;
use std::thread;
use tinyrand_examples::SomeSpecialCondition;

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
Invoking `Default::default()` on a `Rand` initialises it with a constant seed. This is great for repeatability but results in the same run of "random" numbers, which is not what most folks need.

`tinyrand` is a `no_std` crate and, sadly, there is no good, portable way to generate entropy when one cannot make assumptions about the underlying platform. In most applications, one might a clock, but something as trivial as `SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)` mightn't be always available.

If you have an entropy source at your disposal, you could seed an `Rrnd` as so:

```rust
use tinyrand::{Rand, StdRand, Seeded};
let seed = tinyrand_examples::get_seed_from_somewhere(); // some source of entropy

let mut rand = StdRand::seed(seed);
let num = rand.next_u64();
println!("generated {num}");
```

You might also consider using [`getrandom`](https://lib.rs/crates/getrandom), which is a cross-platform method for retrieving entropy data.

If one doesn't care about `no_std`, they shouldn't be bound by its limitations. To seed from the system clock, you can opt in to `std`:

```sh
cargo add tinyrand-std
```

Now, we have a `ClockSeed` at our disposal, which also implements the `Rand` trait. `ClockSeed` derives a `u64` by XORing the upper 64 bits of the nanosecond timestamp (from `SystemTime`) with the lower 64 bits. It's not suitable for cryptographic use but will suffice for most general-purpose applications.

```rust
use tinyrand::{Rand, StdRand, Seeded};
use tinyrand_std::clock_seed::ClockSeed;

let seed = ClockSeed::default().next_u64();
println!("seeding with {seed}");
let mut rand = StdRand::seed(seed);
let num = rand.next_u64();
println!("generated {num}");
```

# Mocking
Good testing coverage can sometimes be hard to achieve; doubly so when applications depend on randomness or other sources of nondeterminism. `tinyrand` comes with a mock RNG that offers fine-grained control over the execution of your code.

The mock uses the `alloc` crate, as it requires heap allocation of closures. As such, the mock is distributed as an opt-in package:

```sh
cargo add tinyrand-alloc
```

At the grassroots level, `Mock` is struct configured with a handful of **delegates**. A delegate is a closure that is invoked by the mock when a particular trait method is called by the system under test. The mock also maintains an internal invocation state that keeps track of the number of times a particular delegate was exercised. So, not only can you mock the behaviour of the `Rand` trait, but also verify the number of types a particular group of related trait methods were called.

The delegates are specified by the test case, while the mock instance is passed to the system under test as a `Rand` implementation. Currently, three delegate types are supported:

1. `FnMut(&State) -> u128` — invoked when one of the `next_uXX()` methods is called on the mock. (`uXX` being one of `u16`, `u32`, `u64`, `u128` or `usize`.) The delegate returns the next "random" number, which may be up to 128 bits wide. The width is designed to accommodate `u128` — the widest type supported by `Rand`. If one of the narrower types is requested, the mock simply returns the lower bits. (E.g., for a `u32`, the mocked value is truncated using `as u32` under the hood.)
2. `FnMut(Surrogate, Probability) -> bool` — invoked when the `next_bool(Probability)` method is called.
3. `FnMut(Surrogate, u128) -> u128` — when either `next_lim` or `next_range` is called.

Starting with the absolute basics, let's mock `next_uXX()` to return a constant. We'll then check how many times our mock got called.

```rust
use tinyrand::Rand;
use tinyrand_alloc::Mock;

let mut rand = Mock::default().with_next_u128(|_| 42);
for _ in 0..10 {
    assert_eq!(42, rand.next_usize()); // always 42
}
assert_eq!(10, rand.state().next_u128_invocations());
```

Although embarrassingly simple, this scenario is actually quite common. The same can be achieved with the `fixed(uXX)` function.

```rust
use tinyrand::Rand;
use tinyrand_alloc::{Mock, fixed};

let mut rand = Mock::default().with_next_u128(fixed(42));
assert_eq!(42, rand.next_usize()); // always 42
```

Since delegates are regular closures, we can bind to variables in the enclosing scope. This gives us almost unlimited control over our mock's behaviour.

```rust
use tinyrand::Rand;
use tinyrand_alloc::Mock;
use core::cell::RefCell;

let val = RefCell::new(3);
let mut rand = Mock::default().with_next_u128(|_| *val.borrow());

assert_eq!(3, rand.next_usize());

// ... later ...
*val.borrow_mut() = 17;
assert_eq!(17, rand.next_usize());
```

The delegate can be reassigned at any point, even after the mock has been created and exercised:

```rust
use tinyrand::Rand;
use tinyrand_alloc::{Mock, fixed};

let mut rand = Mock::default().with_next_u128(fixed(42));
assert_eq!(42, rand.next_usize());

rand = rand.with_next_u128(fixed(88)); // the mock's behaviour is now altered
assert_eq!(88, rand.next_usize());
```

The signature of the `next_u128` delegate takes a `State` reference, which captures the number of times the mock was invoked. (The count is incremented only after the invocation is complete.) Let's write a mock that returns a "random" number derived from the invocation state.

```rust
use tinyrand::Rand;
use tinyrand_alloc::Mock;

let mut rand = Mock::default().with_next_u128(|state| {
    // return number of completed invocations
    state.next_u128_invocations() as u128
});
assert_eq!(0, rand.next_usize());
assert_eq!(1, rand.next_usize());
assert_eq!(2, rand.next_usize());
```

This is useful when we expect the mock to be called several times and each invocation should return a different result. A similar outcome can be achieved with the `counter(Range)` function, which cycles through a specified range of numbers, conveniently wrapping at the boundary:

```rust
use tinyrand::Rand;
use tinyrand_alloc::{Mock, counter};

let mut rand = Mock::default().with_next_u128(counter(5..8));
assert_eq!(5, rand.next_usize());
assert_eq!(6, rand.next_usize());
assert_eq!(7, rand.next_usize());
assert_eq!(5, rand.next_usize()); // start again
```

By supplying just the `next_u128` delegate, we can influence the result of every other method in the `Rand` trait, because they all derive from the _same_ source of randomness and will eventually call our delegate under the hood... in theory! In practice, things are a lot more complicated.

Derived `Rand` methods, such as `next_bool(Probability)`, `next_lim(uXX)` and `next_range(Range)` are backed by different probability distributions. `next_bool`, for example, draws from the Bernoulli distribution, whereas `next_lim` and `next_range` use a scaled uniform distribution with an added debiasing layer. Furthermore, the mapping between the various distributions is an internal implementation detail that is subject to change. The debiasing layer alone has several implementations, optimised for types of varying widths. In other words, the mappings from `next_u128` to `next_bool`, `next_lim` and `next_range` and nontrivial; it's not something you'll want to mock without a calculator and some knowledge of modular arithmetic.

Luckily, `Rand` lets us "bypass" these mapping functions. This is where the other two delegates come in. In the following example, we mock the outcome of `next_bool`.

```rust
use tinyrand::{Rand, Probability};
use tinyrand_alloc::Mock;

let mut rand = Mock::default().with_next_bool(|_, _| false);
if rand.next_bool(Probability::new(0.999999)) {
    println!("very likely");
} else {
    // we can cover this branch thanks to the magic of mocking
    println!("very unlikely");
}
```

The `next_bool` delegate is handed a `Surrogate` struct, which is both a `Rand` implementation and keeper of the invocation state. The surrogate lets us derive `bool`s, as so:

```rust
use tinyrand::{Rand, Probability};
use tinyrand_alloc::Mock;

let mut rand = Mock::default().with_next_bool(|surrogate, _| {
    surrogate.state().next_bool_invocations() % 2 == 0
});
assert_eq!(true, rand.next_bool(Probability::new(0.5)));
assert_eq!(false, rand.next_bool(Probability::new(0.5)));
assert_eq!(true, rand.next_bool(Probability::new(0.5)));
assert_eq!(false, rand.next_bool(Probability::new(0.5)));
```

The surrogate also lets the delegate call the mocked methods from inside the mock.

The last delegate is used to mock both `next_lim` and `next_range` methods, owing to their isomorphism. Under the hood, `next_range` delegates to `next_lim`, such that, for any pair of limit boundaries (`M`, `N`), `M` < `N`, `next_range(M..N)` = `M` + `next_lim(N - M)`. This is how it's all mocked in practice:

```rust
use tinyrand::{Rand, RandRange};
use tinyrand_alloc::Mock;

enum Day {
    Mon, Tue, Wed, Thu, Fri, Sat, Sun
}
const DAYS: [Day; 7] = [Day::Mon, Day::Tue, Day::Wed, Day::Thu, Day::Fri, Day::Sat, Day::Sun];

let mut rand = Mock::default().with_next_lim_u128(|_, _| 6);
let day = &DAYS[rand.next_range(0..DAYS.len())];
assert!(matches!(day, Day::Sun)); // always a Sunday
assert!(matches!(day, Day::Sun)); // yes!!!
```

# Credits
* G. Marsaglia for [Xorshift](https://en.wikipedia.org/wiki/Xorshift).
* Y. Wang, D. B. Romero, D. Lemire and L. Jin for [Wyrand](https://github.com/wangyi-fudan/wyhash/blob/master/Modern%20Non-Cryptographic%20Hash%20Function%20and%20Pseudorandom%20Number%20Generator.pdf).
* R. G. Brown for the [Dieharder](http://webhome.phy.duke.edu/~rgb/General/dieharder.php) test suite.
* D. Lemire for his work on [Fast Random Integer Generation in an Interval](https://arxiv.org/abs/1805.10941).