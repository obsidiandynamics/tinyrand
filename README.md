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

| RNG        | Algorithm | Bandwidth (GB/s) |
|:-----------|:----------|-----------------:|
| `rand`     | ChaCha12  |              2.2 |
| `fastrand` | Wyrand    |              5.1 |
| `tinyrand` | Wyrand    |             14.6 |
| `tinyrand` | Xorshift  |              6.7 |

TL;DR: `tinyrand` is almost 3x faster than `fastrand` and more than 6x faster than `rand`.

# Statistical properties
It's impossible to tell for certain whether an RNG is good; the answer is probabilistic. `tinyrand` (both Wyrand and Xorshift) copes very well against the [Dieharder](http://webhome.phy.duke.edu/~rgb/General/dieharder.php) battery of tests. That means it produces numbers that appear sufficiently random and is likely fit for use in most applications.

`tinyrand` algorithms are not cryptographically secure, meaning it is possible to guess the next random number by observing a sequence of numbers. If you need a CSPRNG, it is strongly suggested that you go with `rand`. Most folks don't, but if you do...

# Getting started
## Add dependency
```sh
cargo add tinyrand
```