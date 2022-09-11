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
/// The given `trial` closure is repeated _n_ times. The resulting p-value is compared
/// against the scaled `significance_level` (by a factor of 1/_n_, in accordance with
/// Bonferroni) to identify rejections.
pub fn bonferroni_correction(
    significance_level: f64,
    n: u16,
    mut trial: impl FnMut() -> f64,
) -> Result<(), Vec<Rejection>> {
    assert!(n > 0);
    assert!(significance_level >= f64::EPSILON);
    assert!(significance_level <= 1.0 - f64::EPSILON);

    let alpha = significance_level / f64::from(n);

    let rejections = (0..n)
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

/// Runs a series of trials using Holm-Bonferroni sequential correction to depress the Type I error rate.
///
/// The given `trial` closure is repeated _n_ times. The resulting p-values are sorted in ascending
/// order. Each _p_<sub>_i_</sub> is subsequently compared against an adjusted α, where α = 1/_i_ * `significance_level`.
pub fn holm_bonferroni_seq_correction(
    significance_level: f64,
    n: u16,
    mut trial: impl FnMut() -> f64,
) -> Result<(), Vec<Rejection>> {
    assert!(n > 0);
    assert!(significance_level >= f64::EPSILON);
    assert!(significance_level <= 1.0 - f64::EPSILON);

    // run the trials, capturing the p-values
    let mut p_values = (0..n)
        .map(|_| trial())
        .collect::<Vec<_>>();

    // arrange p-values in ascending order
    p_values.sort_by(|a, b|a.total_cmp(b));

    // println!("p_values={p_values:?}");

    // compare ordered p-values against the incrementally adjusted alpha
    let num_trials = f64::from(n);
    let rejections = p_values.into_iter().enumerate()
        .map(|(i, p_value)| {
            let alpha = significance_level / (num_trials - i as f64);
            Rejection { alpha, p_value }
        })
        .filter(|rejection| {
            rejection.p_value < rejection.alpha
        })
        .collect::<Vec<_>>();

    if rejections.is_empty() {
        Ok(())
    } else {
        Err(rejections)
    }
}

/// Integrates the discrete probabilities from the Binomial PMF that are more likely than
/// that where _K_=_k_.
///
/// `n` — number of experiments in the sequence.
/// `p` — probability of success (equivalently, weight of the coin, where `p` > 0.5 is biased towards heads).
pub fn integrate_binomial(n: u16, p: f64, k: u16) -> f64 {
    // find the probability of k successes in n experiments
    let outcome_prob = binomial_pmf(k, n, p);

    // sum the probabilities of all other outcomes which are more probable than those with k successes
    (0..=n)
        .map(|k| binomial_pmf(k, n, p))
        .filter(|&p| p > outcome_prob)
        .sum::<f64>()
        .min(1.0)
}

/// Integrates the discrete probabilities from the Poisson PMF that are more likely than that
/// where _K_=_k_.
///
/// `lambda` — the arrival rate.
pub fn integrate_poisson(lambda: f64, k: u16) -> f64 {
    let outcome_prob = poisson_pmf(k, lambda);
    let dist_from_lambda = (f64::from(k) - lambda).abs();

    // max_events is the upper bound in our integral range; there is no point going above it as all
    // the probabilities there are lower than outcome_prob
    let max_events = (lambda + dist_from_lambda).ceil() as u16;
    (0..=max_events)
        .map(|k| poisson_pmf(k, lambda))
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

/// Obtains the Poisson Probability Mass Function.
///
/// `lambda` — the arrival rate.
/// `k` — number of events.
pub fn poisson_pmf(k: u16, lambda: f64) -> f64 {
    lambda.powi(i32::from(k)) * (-lambda).exp() / fact(k) as f64
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