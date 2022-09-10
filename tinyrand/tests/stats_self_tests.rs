//! Self tests for the `stats` module.
//!
//! If these tests were to be added as unit tests of `stats`, they would be repeated for each
//! integration test that uses `stats`.

use crate::stats::{binomial_pmf, fact, fact_trunc, ncr, poisson_pmf};

pub mod stats;

#[test]
fn test_bernoulli_pmf() {
    assert_float_epsilon(0.059535, binomial_pmf(4, 6, 0.3));
}

#[test]
fn test_poisson_pmf() {
    assert_float_epsilon(0.14037389581428056, poisson_pmf(3, 5.0));
    assert_float_epsilon(0.1754673697678507, poisson_pmf(5, 5.0));
}

fn assert_float_epsilon(lhs: f64, rhs: f64) {
    assert_float(lhs, rhs, f64::EPSILON)
}

fn assert_float(lhs: f64, rhs: f64, delta: f64) {
    assert!((rhs - lhs).abs() <= delta, "lhs={lhs} rhs={rhs}");
}

#[test]
fn test_fact() {
    assert_eq!(1, fact(0));
    assert_eq!(1, fact(1));
    assert_eq!(2, fact(2));
    assert_eq!(6, fact(3));
}

#[test]
fn test_fact_trunc() {
    assert_eq!(1, fact_trunc(0, 0));
    assert_eq!(1, fact_trunc(0, 1));
    assert_eq!(1, fact_trunc(1, 1));
    assert_eq!(2, fact_trunc(1, 2));
    assert_eq!(1, fact_trunc(2, 2));
    assert_eq!(6, fact_trunc(1, 3));
    assert_eq!(3, fact_trunc(2, 3));
    assert_eq!(1, fact_trunc(3, 3));
    assert_eq!(24, fact_trunc(1, 4));
    assert_eq!(12, fact_trunc(2, 4));
    assert_eq!(4, fact_trunc(3, 4));


    assert_eq!(fact(0) / fact(0), fact_trunc(0, 0));
    assert_eq!(fact(1) / fact(0), fact_trunc(0, 1));
    assert_eq!(fact(1) / fact(1), fact_trunc(1, 1));
    assert_eq!(fact(2) / fact(1), fact_trunc(1, 2));
    assert_eq!(fact(2) / fact(2), fact_trunc(2, 2));
    assert_eq!(fact(3) / fact(1), fact_trunc(1, 3));
    assert_eq!(fact(3)/ fact(2), fact_trunc(2, 3));
    assert_eq!(fact(3) / fact(3), fact_trunc(3, 3));
    assert_eq!(fact(4)/ fact(1), fact_trunc(1, 4));
    assert_eq!(fact(4) / fact(2), fact_trunc(2, 4));
    assert_eq!(fact(4)/ fact(3), fact_trunc(3, 4));
}

#[test]
fn test_ncr() {
    assert_eq!(1, ncr(1, 0));
    assert_eq!(1, ncr(1, 1));
    assert_eq!(1, ncr(2, 0));
    assert_eq!(2, ncr(2, 1));
    assert_eq!(1, ncr(2, 2));
    assert_eq!(1, ncr(3, 0));
    assert_eq!(3, ncr(3, 1));
    assert_eq!(3, ncr(3, 2));
    assert_eq!(1, ncr(3, 3));
    assert_eq!(1, ncr(4, 0));
    assert_eq!(4, ncr(4, 1));
    assert_eq!(6, ncr(4, 2));
    assert_eq!(4, ncr(4, 3));
    assert_eq!(1, ncr(4, 4));
    assert_eq!(1, ncr(5, 0));
    assert_eq!(5, ncr(5, 1));
    assert_eq!(10, ncr(5, 2));
    assert_eq!(10, ncr(5, 3));
    assert_eq!(5, ncr(5, 4));
    assert_eq!(1, ncr(5, 5));
}