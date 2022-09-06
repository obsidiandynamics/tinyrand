//! Mock RNG for testing.

use core::cell::{RefCell};
use core::ops::{Range};
use crate::Rand;

/// Mock invocation state.
#[derive(Default)]
pub struct State {
    next_u64_invocations: u64,
}

impl State {
    /// Obtains the number of invocations of the [`Rand::next_u64`] method.
    pub fn next_u64_invocations(&self) -> u64 {
        self.next_u64_invocations
    }
}

// Mock RNG, initialised with a delegate closure.
pub struct Mock<D: FnMut(&State) -> u64> {
    state: State,
    delegate: D,
}

impl<D: FnMut(&State) -> u64> Mock<D> {
    /// Creates a new mock with the supplied delegate closure.
    ///
    /// # Examples
    /// ```
    /// use tinyrand::Rand;
    /// use tinyrand::mock::Mock;
    /// let mut mock = Mock::new(|_| 42);
    /// assert_eq!(42, mock.next_u64());
    /// ```
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
    fn next_u64(&mut self) -> u64 {
        let delegate = &mut self.delegate;
        let r = delegate(&self.state);
        self.state.next_u64_invocations += 1;
        r
    }
}

/// A pre-canned delegate that counts in the given range, wrapping around when it reaches
pub fn __counter<T, S>(range: Range<T>) -> impl FnMut(&S) -> T
    where
        T: Copy + Next + Eq
{
    let mut current = range.start;
    move |_| {
        let c = current;
        let next = current.next();
        current = if next == range.end { range.start } else { next };
        c
    }
}

/// Something that has a successor value.
pub trait Next {
    #[must_use]
    fn next(self) -> Self;
}

impl Next for u64 {
    fn next(self) -> Self {
        self + 1
    }
}

impl Next for u128 {
    fn next(self) -> Self {
        self + 1
    }
}

/// A pre-canned delegate that always parrots a given value.
pub fn __fixed<T: Copy, S>(val: T) -> impl FnMut(&S) -> T {
    move |_| val
}

/// Accessor and mutator methods for [`RefCell`].
pub trait RefCellExt<T> {
    fn get(&self) -> T;

    fn set(&self, val: T);
}

impl<T: Copy> RefCellExt<T> for RefCell<T> {
    fn get(&self) -> T {
        *self.borrow()
    }

    fn set(&self, val: T) {
        *self.borrow_mut() = val;
    }
}

/// A pre-canned delegate that parrots the value contained in the given cell.
pub fn __echo<T: Copy, S>(cell: &RefCell<T>) -> impl FnMut(&S) -> T + '_ {
    |_| *cell.borrow()
}

#[cfg(test)]
mod tests;