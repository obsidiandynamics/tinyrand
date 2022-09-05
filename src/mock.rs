//! Mock RNG for testing.

use core::ops::Range;
use crate::Rand;

/// Mock state.
#[derive(Default)]
pub struct State {
    invocations: u64,
}

impl State {
    /// Obtains the number of invocations of the [`Rand::next_u64`] method.
    pub fn invocations(&self) -> u64 {
        self.invocations
    }
}

// Mock RNG, initialised with a delegate closure.
pub struct Mock<D: FnMut(&State) -> u64> {
    state: State,
    delegate: D,
}

impl<D: FnMut(&State) -> u64> Mock<D> {
    /// Creates a new mock with the supplied delegate closure.
    #[inline(always)]
    pub fn new(delegate: D) -> Self {
        Self {
            state: State::default(),
            delegate
        }
    }

    /// Obtains the underlying mock state.
    pub fn state(&self) -> &State {
        &self.state
    }
}

impl<D: FnMut(&State) -> u64> Rand for Mock<D> {
    /// Delegates to the underlying closure and increments the `state.invocations` counter
    /// _after_ the closure returns.
    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        let delegate = &mut self.delegate;
        let r = delegate(&self.state);
        self.state.invocations += 1;
        r
    }
}

/// A pre-canned delegate that counts in the given range, wrapping around when it reaches
/// the end.
///
/// # Examples
/// ```
/// use tinyrand::{Mock, Rand};
/// use tinyrand::mock::counter;
/// let mut mock = Mock::new(counter(5..8));
/// assert_eq!(5, mock.next_u64());
/// assert_eq!(6, mock.next_u64());
/// assert_eq!(7, mock.next_u64());
/// assert_eq!(5, mock.next_u64());
/// ```
pub fn counter(range: Range<u64>) -> impl FnMut(&State) -> u64 {
    let mut current = range.start;
    move |_| {
        let c = current;
        let next = current + 1;
        current = if next == range.end { range.start } else { next };
        c
    }
}

/// A pre-canned delegate that always parrots a given value.
///
/// # Examples
/// ```
/// use tinyrand::{Mock, Rand};
/// use tinyrand::mock::constant;
/// let mut mock = Mock::new(constant(42));
/// assert_eq!(42, mock.next_u64());
/// assert_eq!(42, mock.next_u64());
/// ```
pub fn constant(val: u64) -> impl FnMut(&State) -> u64 {
    move |_| val
}

#[cfg(test)]
mod tests;