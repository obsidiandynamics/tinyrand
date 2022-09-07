//! Mock RNG for internal testing of this crate.

use core::cell::{RefCell};
use core::ops::{Range};
use crate::{Rand, rand64};
use crate::rand64::Rand64;

/// Mock invocation state.
#[derive(Default)]
pub struct State {
    next_u64_invocations: u64,
}

impl State {
    /// Obtains the number of invocations of the [`Rand::next_u64`] method.
    fn next_u64_invocations(&self) -> u64 {
        self.next_u64_invocations
    }
}

/// Mock RNG, initialised with a delegate closure.
pub struct TestMock<D: FnMut(&State) -> u64> {
    state: State,
    delegate: D,
}

impl<D: FnMut(&State) -> u64> TestMock<D> {
    /// Creates a new test_mock with the supplied delegate closure.
    pub fn new(delegate: D) -> Self {
        Self {
            state: State::default(),
            delegate
        }
    }

    /// Obtains the underlying test_mock state.
    pub fn state(&self) -> &State {
        &self.state
    }
}

impl<D: FnMut(&State) -> u64> Rand for TestMock<D> {
    fn next_u16(&mut self) -> u16 {
        rand64::next_u16(self)
    }

    fn next_u32(&mut self) -> u32 {
        rand64::next_u32(self)
    }

    /// Delegates to the underlying closure and increments the `state.invocations` counter
    /// _after_ the closure returns.
    fn next_u64(&mut self) -> u64 {
        let delegate = &mut self.delegate;
        let r = delegate(&self.state);
        self.state.next_u64_invocations += 1;
        r
    }

    fn next_u128(&mut self) -> u128 {
        rand64::next_u128(self)
    }
}

impl<D: FnMut(&State) -> u64> Rand64 for TestMock<D> {}

/// A pre-canned delegate that counts in the given range, wrapping around when it reaches
/// the end.
pub fn counter<T, S>(range: Range<T>) -> impl FnMut(&S) -> T
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
pub fn fixed<T: Copy, S>(val: T) -> impl FnMut(&S) -> T {
    move |_| val
}

/// A pre-canned delegate that parrots the value contained in the given cell.
pub fn echo<T: Copy, S>(cell: &RefCell<T>) -> impl FnMut(&S) -> T + '_ {
    |_| *cell.borrow()
}

#[cfg(test)]
mod tests;