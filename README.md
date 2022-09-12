# `tinyrand`
Lightweight RNG specification and several ultrafast implementations in Rust. `tinyrand` is `no_std` and doesn't use a heap allocator.

[![Crates.io](https://img.shields.io/crates/v/tinyrand?style=flat-square&logo=rust)](https://crates.io/crates/tinyrand)
[![docs.rs](https://img.shields.io/badge/docs.rs-tinyrand-blue?style=flat-square&logo=docs.rs)](https://docs.rs/tinyrand)
[![Build Status](https://img.shields.io/github/workflow/status/obsidiandynamics/tinyrand/Cargo%20build?style=flat-square&logo=github)](https://github.com/obsidiandynamics/tinyrand/actions/workflows/master.yml)
[![codecov](https://img.shields.io/codecov/c/github/obsidiandynamics/tinyrand/master?style=flat-square&logo=codecov)](https://codecov.io/gh/obsidiandynamics/tinyrand)
![no_std](https://img.shields.io/badge/linking-no__std-9cf?style=flat-square)

# Why `tinyrand`?
* It's very small and doesn't need `std`, meaning it's embeddable — it runs on microcontrollers and bare-metal (no OS) environments.
* It's very fast. It comes bundled with [Xorshift](https://en.wikipedia.org/wiki/Xorshift), [SplitMix](https://dl.acm.org/doi/10.1145/2660193.2660195) and [Wyrand](https://github.com/wangyi-fudan/wyhash/blob/master/Modern%20Non-Cryptographic%20Hash%20Function%20and%20Pseudorandom%20Number%20Generator.pdf).
* The RNG behaviour is concisely specified as a handful of traits, independent of the underlying implementations. It makes it easy to swap implementations.
* It comes with [`Mock`](https://docs.rs/tinyrand-alloc/latest/tinyrand_alloc/mock/index.html) for testing code that depends on random numbers. That is, if you care about code coverage.

# Performance
Below is a comparison of several notable PRNGs.

| RNG        | Algorithm | Bandwidth (GB/s) |                                                                                       |
|:-----------|:----------|-----------------:|:--------------------------------------------------------------------------------------|
| `rand`     | ChaCha12  |              2.4 | <img src="https://via.placeholder.com/12/FF5733/FF5733.png" width="24" height="12"/>  |
| `tinyrand` | SplitMix  |              6.5 | <img src="https://via.placeholder.com/12/33FFE0/33FFE0.png" width="65" height="12"/>  |
| `tinyrand` | Xorshift  |              6.7 | <img src="https://via.placeholder.com/12/33FFE0/33FFE0.png" width="67" height="12"/>  |
| `fastrand` | Wyrand    |              7.5 | <img src="https://via.placeholder.com/12/FFC733/FFC733.png" width="75" height="12"/>  |
| `tinyrand` | Wyrand    |             14.6 | <img src="https://via.placeholder.com/12/33FFE0/33FFE0.png" width="146" height="12"/> |

TL;DR: `tinyrand` is 2x faster than `fastrand` and 6x faster than `rand`.

# Statistical properties
It's impossible to tell for certain whether a certain PRNG is good; the answer is probabilistic. All three algorithms stand up well against the [Dieharder](http://webhome.phy.duke.edu/~rgb/General/dieharder.php) barrage of tests, but Wyrand and SplitMix are a little better than Xorshift. (Tested on 30.8 billion samples.) This means `tinyrand` produces numbers that appear sufficiently random and is likely fit for use in most applications.

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

The `tinyrand-std` crate also includes a seeded, thread-local `Rand` implementation:

```rust
use tinyrand::Rand;
use tinyrand_std::thread_rand;

let mut rand = thread_rand();
let num = rand.next_u64();
println!("generated {num}");
```

# Mocking
Good testing coverage can sometimes be hard to achieve; doubly so when applications depend on randomness or other sources of nondeterminism. `tinyrand` comes with a mock RNG that offers fine-grained control over the execution of your code.

The mock uses the `alloc` crate, as it requires heap allocation of closures. As such, the mock is distributed as an opt-in package:

```sh
cargo add tinyrand-alloc
```

At the grassroots level, [`Mock`](https://docs.rs/tinyrand-alloc/latest/tinyrand_alloc/mock/index.html) is struct configured with a handful of **delegates**. A delegate is a closure that is invoked by the mock when a particular trait method is called by the system under test. The mock also maintains an internal invocation state that keeps track of the number of times a particular delegate was exercised. So, not only can you mock the behaviour of the `Rand` trait, but also verify the number of types a particular group of related trait methods were called.

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

# How is `tinyrand` tested?
This section briefly describes the `tinyrand` testing approach. It is aimed at those who —

* Want to know whether they are getting "the real deal";
* Wish to understand how PRNGs can be practically tested; and
* Are wondering what it is meant by "likely fit for use in most applications".

The `tinyrand` testing process is split into four tiers:

1. Unit tests are used to ensure 100% code coverage and assert the elemental sanity of `tinyrand`. In other words, every line of code is exercised at least once, fundamental expectations are upheld and there are _likely_ no trivial defects.
2. Synthetic benchmarks.
3. Statistical tests are then used to verify specific properties of the PRNGs that make up `tinyrand`. These are formal hypothesis tests that assume that the source is random (the null hypothesis), and look for evidence to dispel this assumption (the alternate hypothesis).
4. The [Dieharder](http://webhome.phy.duke.edu/~rgb/General/dieharder.php) statistical test suite.

## Unit tests
The unit tests are not aimed at asserting numerical qualities; they are purely functional in nature. Objectives include —

* **Coverage testing.** `tinyrand` is built on the philosophy that if a line of code is not provably exercised, it should be removed. There are no exceptions to this rule.
* **Seeding and state management.** Every PRNG must be able to maintain state between invocations and some can be initialised from a user-supplied seed. Tests exist to verify this.
* **Domain transforms.** A PRNG, at minimum, generates uniformly distributed values in some _a priori_ range. In practice, we need random numbers in some specific range that is useful to our application. Also, we may stipulate that some values appear more frequently than others; for example, the weighting of the `true` outcome versus `false` in the generation of `bool`s. The functions for mapping from the uniform distribution to a custom one are nontrivial and require a debiasing layer. `tinyrand` uses different debiasing methods depending on the word width. The purpose of the domain transform tests is to verify that this functionality is working as expected and rejection sampling is taking place. It doesn't verify the numerical properties of debiasing, however.

## Synthetic benchmarks
The synthetic benchmarks are used to exercise the hot paths of the `tinyrand` PRNGs, comparing the results to peer libraries. The benchmarks test the generation of numbers at various word lengths, transforms/debiasing and the generation of weighted `bool`s. A subset of these benchmarks is also included in the CI tests, making it a little easier to compare the performance of `tinyrand` across commit versions.

## Statistical hypothesis testing
`tinyrand` comes bundled with an integrated statistical testing suite, inspired by the likes of [Diehard](https://en.wikipedia.org/wiki/Diehard_tests), [Dieharder](http://webhome.phy.duke.edu/~rgb/General/dieharder.php) and [NIST SP 800-22](https://csrc.nist.gov/publications/detail/sp/800-22/rev-1a/final). The `tinyrand` suite is admittedly much smaller than any of these tests; the intention is not to replicate the already substantial and readily accessible work in this area, but to create a safety net that is both very effective at detecting common anomalies and fast enough to be run at every commit.

The following tests are included.

* **Bit flip**: Conducts a series of Bernoulli trials on a `Rand` instance by masking the value of a single bit, verifying that the number of times the bit is set to 1 is within the expected range. For each subsequent trial, the mask is shifted by one to the left and the hypothesis is retested. The test proceeds over several cycles; each cycle comprising 64 Bernoulli trials (one for each bit of a `u64`).
* **Coin flip**: Whereas _bit flip_ works at the level of individual bits in a random word and is unweighted (or equally weighted), _coin flip_ uses the Bernoulli distribution to obtain a `bool` with a chosen probability from a 64-bit unsigned word. The test comprises a series of Bernoulli trials with a different (randomly chosen) weighting on each trial, simulating a run of coin flips. Within each trial, H0 asserts that the source is random. (I.e., the number of 'heads' falls within a statistically acceptable interval.)
* **Collision**: A series of trials with a different (randomly chosen) integer generation range on each trial. Within each trialled range, one random number is chosen as the control value. A series of random numbers (sampled from the same range) is then produced and the number of collisions with the control value is counted. By H0, the collisions should follow a Poisson process with λ as the expected collision rate.
* **Monobit**: Counts the number of bits in 32-bit words, taken by alternating between the MSB and LSB segments of generated `u64`s in separate trials. In each trial, we assume that the values of individual bits are IID with probability of 0.5, verifying that the number of times the bit is set to 1 is within the expected range. For a random source, the number of 1s (and 0s) follows a Bernoulli process.
* **Sum convergence**: A series of trials with a different (randomly chosen) integer generation range on each trial. Within each trial, H0 asserts that the source is random. (I.e., the sum of the sampled values falls within a statistically acceptable range.) The Gaussian distribution is used as an [approximation of the Irwin-Hall distribution](https://en.wikipedia.org/wiki/Irwin%E2%80%93Hall_distribution#Approximating_a_Normal_distribution), with the unscaled mean and variance parameters set to _n_/2 and _n_/12 respectively.
* **Lagged sum convergence**: Similar to the standard _sum convergence_, but skipping a fixed number of samples in computing the sum. This test looks for lagged autocorrelations in the PRNG, which are otherwise difficult to detect. The lag is set to small powers of two. A _sum convergence_ test is a limiting case of the _lagged sum convergence_ test, with lag set to zero.

Each of `tinyrand`'s tests is exercised not only against its own PRNGs, but also against intentionally faulty implementations, which are used to verify the efficacy of the test. The tests must consistently fail to reject H0 for the correct PRNGs and accept H1 for the faulty ones.

The statistical tests are themselves seeded from random values. Randomness is used to seed the PRNGs under test (every trial is independently seeded), assign weightings to Bernoulli experiments, select integer ranges for testing transform functions and debiasing, control values for testing collisions, and so forth. We use the `rand` package as the control PRNG so that a defect in `tinyrand` cannot inadvertently subvert a test in a way that masks itself. Tests are seeded so that, while they appear to be on a random excursion through the parameter space, their choice of parameters is entirely deterministic and hence repeatable. This is essential due to the possibility of Type I error (incorrectly rejecting the null hypothesis), which mustn't be allowed to occur intermittently, especially in CI environments. In other words, _testing of randomness cannot be left to chance_.

One way of testing the randomness hypothesis is to select a set of parameters (e.g., the integer generation range `M`..`N` or the probability of obtaining `true` from a Bernoulli distribution) and to perform a long run, seeking anomalies in a large random sample. The rationale is that the larger the sample, the more likely it will contain a detectable anomaly. This is generally not very effective for spotting certain kinds of anomalies that may affect PRNGs only under very specific conditions. For example, a poorly written debiasing function may still perform well for most small integer ranges and even some large ones (those that are close to powers of two). If the test picks parameters unfavourably, it may not find anomalies no matter how exhaustively it tests those parameters. 

A much better way of testing PRNG is to introduce diversity into the testing regime — conducting a large number of small trials with different parameters rather than one, very large trial. This is precisely what the `tinyrand` statistical tests do — conduct several trials with randomly (but deterministically) selected parameters. This immediately exposes the [multiple comparisons problem](https://en.wikipedia.org/wiki/Multiple_comparisons_problem). Consider an _a priori_ ideal PRNG. It will frequently generate numbers that will appear "random" according to some agreed measure. But occasionally, it will produce output that will appear nonrandom by the same measure. Even an ideal source will produce a very long run of ones or zeros, for example. In fact, failing to do so would also render it nonrandom. Unfortunately, this will produce a p-value that will fail even the most relaxed test... at some point. This is a problem for single hypothesis testing, but it is proportionally exacerbated in multiple hypothesis testing.

The `tinyrand` built-in tests address this problem using the Holm-Bonferroni sequential correction method. Holm-Bonferroni correction suppresses Type I errors while maintaining good statistical power — suppression of Type II errors. It appears to perform well for `tinyrand`'s needs, especially seeing that the number trials is generally kept in the 100—1000 range. (`tinyrand` tests are designed to be very quick, which places a practical bound on the number of trials — ideally, all statistical tests should complete within a few seconds for them to be mandated as part of routine development flow.)

## Dieharder tests
[Dieharder](http://webhome.phy.duke.edu/~rgb/General/dieharder.php) test suite extends Marsaglia's original [Diehard](https://en.wikipedia.org/wiki/Diehard_tests) battery of tests. It is bundled with a large number of tests and takes a long time (~1 hour) to complete. `tinyrand` has a utility for pumping random output to Dieharder, which is typically run on an ad hoc basis. The Dieharder battery should be run when a PRNG undergoes a material change, which is rare — once a PRNG algorithm is implemented, it generally remains untouched unless it is either refactored or some defect is found. Dieharder is arguably more useful for building and testing experimental PRNGs with `tinyrand`. The other three tiers of tests are sufficient for the maintenance of the `tinyrand` package.

To run `tinyrand` against Dieharder:

```sh
cargo run --release --bin random -- wyrand 42 binary 1T | dieharder -g 200 -a
```

The above command uses the Wyrand PRNG, seeded with the number 42, generating binary output over 1 trillion 64-bit words. It's `stdout` is pumped to `dieharder`. (In practice, Dieharder will consume under 31 billion numbers.)

A word of caution: Dieharder does not have a mechanism for dealing with Type I errors in multiple hypothesis testing — partially because the tests differ in type, not just in parameters. Dieharder limits hypothesis testing to the scope of an individual test; there is no overarching hypothesis which classifies a PRNG as either fit or unfit based on the number of passed tests, or otherwise adjusts the confidence level to account for Type I errors.

# Credits
* G. Marsaglia for [Xorshift](https://en.wikipedia.org/wiki/Xorshift).
* Y. Wang, D. B. Romero, D. Lemire and L. Jin for [Wyrand](https://github.com/wangyi-fudan/wyhash/blob/master/Modern%20Non-Cryptographic%20Hash%20Function%20and%20Pseudorandom%20Number%20Generator.pdf).
* R. G. Brown for the [Dieharder](http://webhome.phy.duke.edu/~rgb/General/dieharder.php) test suite.
* D. Lemire for his work on [Fast Random Integer Generation in an Interval](https://arxiv.org/abs/1805.10941).