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
/// that where _K_=_k_.
///
/// `n` — number of experiments in the sequence.
/// `p` — probability of success (equivalently, weight of the coin, where `p` > 0.5 is biased towards heads).
pub fn integrate_binomial_probs(n: u16, p: f64, k: u16) -> f64 {
    // find the probability of k successes in n experiments
    let outcome_prob = binomial_pmf(k, n, p);

    // sum the probabilities of all other outcomes which are more probable than those with k successes
    (0..=n)
        .map(|k| binomial_pmf(k, n, p))
        .filter(|&p| p > outcome_prob)
        .sum::<f64>()
        .min(1.0)
}

/// Obtains the Binomial Probability Mass Function.
///
/// `k` — number of success outcomes (equivalently, the number of heads).
/// `n` — number of experiments in the sequence (equivalently, the number of coin flips).
/// `p` — probability of success (equivalently, the weight of the coin, where `p` > 0.5 is biased towards heads).
pub fn binomial_pmf(k: u16, n: u16, p: f64) -> f64 {
    ncr(n, k) as f64 * p.powi(k as i32) * (1.0 - p).powi((n - k) as i32)
}

/// Calculates <sup>n</sup>C<sub>r</sub>.
pub fn ncr(n: u16, r: u16) -> u128 {
    assert!(n >= r);
    fact_trunc(r, n) / fact(n - r)
}

/// Calculates n!.
pub fn fact(n: u16) -> u128 {
    let mut fact = 1;
    for i in 2..=u128::from(n) {
        fact *= i;
    }
    fact
}

/// Calculates n!/m!.
pub fn fact_trunc(m: u16, n: u16) -> u128 {
    let mut fact = 1;
    for i in u128::from(m + 1)..=u128::from(n) {
        fact *= i;
    }
    fact
}