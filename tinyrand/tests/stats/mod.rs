//! Utilities for statistical hypothesis testing, combinatorics and distributions.

/// Describes the rejection of a specific trial.
#[derive(Debug)]
pub struct Rejection {
    /// The significance level at which H0 was rejected.
    pub alpha: f64,

    /// The p-value of the test.
    pub p_value: f64,
}

/// Runs a series of trials using Bonferroni correction to depress the Type I error rate.
///
/// The given `trial` closure is repeated `num_trials` times. The resulting p-value is compared
/// against the scaled `significance_level` (by a factor of 1/`num_trials`, in accordance with
/// Bonferroni) to identify rejections.
pub fn bonferroni_correction(
    significance_level: f64,
    num_trials: u16,
    mut trial: impl FnMut() -> f64,
) -> Result<(), Vec<Rejection>> {
    assert!(num_trials > 0);
    assert!(significance_level >= f64::EPSILON);
    assert!(significance_level <= 1.0 - f64::EPSILON);

    let alpha = significance_level / f64::from(num_trials);

    let rejections = (0..num_trials)
        .map(|_| trial())
        .filter(|&p_value| p_value < alpha)
        .collect::<Vec<_>>();

    if rejections.is_empty() {
        Ok(())
    } else {
        let rejections = rejections
            .into_iter()
            .map(|p_value| Rejection { alpha, p_value })
            .collect();
        Err(rejections)
    }
}

/// Integrates the probabilities of all Bernoulli trial outcomes that are more likely than
/// that where `K=k`.
///
/// `n` — number of experiments in the sequence.
/// `w` — probability of success (equivalently, weight of the coin, where `w` > 0.5 is biased towards heads).
pub fn integrate_bernoulli_outcome_probs(n: u16, w: f64, k: u16) -> f64 {
    let outcome_prob = bernoulli_pmf(k, n, w);
    (0..=n)
        .map(|k| bernoulli_pmf(k, n, w))
        .filter(|&p| p > outcome_prob)
        .sum::<f64>()
        .min(1.0)
}

/// Obtains the Bernoulli Probability Mass Function.
///
/// `k` — number of success outcomes (equivalently, 'heads').
/// `n` — number of experiments in the sequence.
/// `w` — probability of success (equivalently, weight of the coin, where `w` > 0.5 is biased towards heads).
pub fn bernoulli_pmf(k: u16, n: u16, w: f64) -> f64 {
    ncr(n, k) as f64 * w.powi(k as i32) * (1.0 - w).powi((n - k) as i32)
}

/// Calculates <sup>n</sup>C<sub>r</sub>.
pub fn ncr(n: u16, r: u16) -> u128 {
    assert!(n >= r);
    fact_trunc(n - r, n) / fact(r)
}

/// Calculates n!.
pub fn fact(n: u16) -> u128 {
    let mut fact = 1;
    for i in 2..=u128::from(n) {
        fact *= i;
    }
    fact
}

/// Calculates n!/(n-m)!.
pub fn fact_trunc(m: u16, n: u16) -> u128 {
    let mut fact = 1;
    for i in u128::from(m + 1)..=u128::from(n) {
        fact *= i;
    }
    fact
}