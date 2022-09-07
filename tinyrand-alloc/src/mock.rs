//! Mock RNG for testing.

use alloc::boxed::Box;
use core::cell::RefCell;
use core::ops::Range;
use tinyrand::{Probability, Rand};

/// Mock invocation state.
#[derive(Default)]
pub struct State {
    next_u128_invocations: u64,
    next_bool_invocations: u64,
    next_lim_u128_invocations: u64,
}

impl State {
    /// Obtains the number of invocations of the [`Rand::next_u128`] method.
    pub fn next_u128_invocations(&self) -> u64 {
        self.next_u128_invocations
    }

    /// Obtains the number of invocations of the [`Rand::next_bool`] method.
    pub fn next_bool_invocations(&self) -> u64 {
        self.next_bool_invocations
    }

    /// Obtains the number of invocations of the [`Rand::next_lim_u128`] method.
    pub fn next_lim_u128_invocations(&self) -> u64 {
        self.next_lim_u128_invocations
    }
}

/// Encapsulates the state of the mock and a reference to the `next_u128` delegate, so
/// that it can be invoked from inside the mock by, for example, another delegate.
pub struct Surrogate<'a, 'd> {
    state: &'a mut State,
    next_u128_delegate: &'a mut Box<dyn FnMut(&State) -> u128 + 'd>,
}

impl Surrogate<'_, '_> {
    /// Obtains the mock state.
    pub fn state(&self) -> &State {
        self.state
    }
}

impl<'a, 'd> Rand for Surrogate<'a, 'd> {
    fn next_u64(&mut self) -> u64 {
        self.next_u128() as u64
    }

    fn next_u128(&mut self) -> u128 {
        let r = (self.next_u128_delegate)(self.state);
        self.state.next_u128_invocations += 1;
        r
    }
}

/// Mock RNG, containing invocation state and delegate closures.
pub struct Mock<'d> {
    state: State,
    next_u128_delegate: Box<dyn FnMut(&State) -> u128 + 'd>,
    next_bool_delegate: Box<dyn FnMut(Surrogate, Probability) -> bool + 'd>,
    next_lim_u128_delegate: Box<dyn FnMut(Surrogate, u128) -> u128 + 'd>,
}

impl<'a> Default for Mock<'a> {
    fn default() -> Self {
        Self {
            state: State::default(),
            next_u128_delegate: Box::new(fixed(0)),
            next_bool_delegate: Box::new(|mut surrogate, p| Rand::next_bool(&mut surrogate, p)),
            next_lim_u128_delegate: Box::new(|mut surrogate, lim| {
                Rand::next_lim_u128(&mut surrogate, lim)
            }),
        }
    }
}

impl<'d> Mock<'d> {
    /// Assigns a [`Rand::next_u128`] delegate to the mock. I.e., when the [`Rand::next_u128`]
    /// method is invoked on the mock (directly, or via another method), it will delegate to
    /// the given closure.
    ///
    /// # Examples
    /// ```
    /// use tinyrand::Rand;
    /// use tinyrand_alloc::Mock;
    /// let mut mock = Mock::default()
    ///     .with_next_u128(|_| 42);
    /// assert_eq!(42, mock.next_usize());
    /// assert_eq!(42, mock.next_u64());
    /// assert_eq!(42, mock.next_u128());
    /// ```
    #[must_use]
    pub fn with_next_u128(mut self, delegate: impl FnMut(&State) -> u128 + 'd) -> Self {
        self.next_u128_delegate = Box::new(delegate);
        self
    }

    /// Assigns a [`Rand::next_bool`] delegate to the mock. I.e., when the [`Rand::next_bool`]
    /// method is invoked on the mock, it will delegate to the given closure.
    ///
    /// # Examples
    /// ```
    /// use tinyrand::{Probability, Rand};
    /// use tinyrand_alloc::Mock;
    /// let mut mock = Mock::default()
    ///     .with_next_bool(|_, _| true);
    /// assert!(mock.next_bool(Probability::new(0.01)));
    /// ```
    #[must_use]
    pub fn with_next_bool(
        mut self,
        delegate: impl FnMut(Surrogate, Probability) -> bool + 'd,
    ) -> Self {
        self.next_bool_delegate = Box::new(delegate);
        self
    }

    /// Assigns a [`Rand::next_lim_u128`] delegate to the mock. I.e., when the [`Rand::next_lim_u128`]
    /// method is invoked on the mock, it will delegate to the given closure. This delegate can be
    /// used to effectively mock `Rand::next_lim` and `Rand::next_range` methods.
    ///
    /// # Examples
    /// ```
    /// use tinyrand::{Rand, RandRange};
    /// use tinyrand_alloc::Mock;
    /// let mut mock = Mock::default()
    ///     .with_next_lim_u128(|_, _| 17);
    /// assert_eq!(17, mock.next_lim_u64(66));
    /// assert_eq!(27, mock.next_range(10..100u16));
    /// ```
    #[must_use]
    pub fn with_next_lim_u128(
        mut self,
        delegate: impl FnMut(Surrogate, u128) -> u128 + 'd,
    ) -> Self {
        self.next_lim_u128_delegate = Box::new(delegate);
        self
    }

    /// Obtains the underlying mock state.
    pub fn state(&self) -> &State {
        &self.state
    }
}

impl Rand for Mock<'_> {
    fn next_u64(&mut self) -> u64 {
        self.next_u128() as u64
    }

    /// Delegates to the underlying closure and increments the `state.next_u128_invocations` counter
    /// _after_ the closure returns.
    fn next_u128(&mut self) -> u128 {
        let next_u64_delegate = &mut self.next_u128_delegate;
        let r = next_u64_delegate(&self.state);
        self.state.next_u128_invocations += 1;
        r
    }

    /// Delegates to the underlying closure and increments the `state.next_bool_invocations` counter
    /// _after_ the closure returns.
    fn next_bool(&mut self, p: Probability) -> bool {
        let surrogate = Surrogate {
            next_u128_delegate: &mut self.next_u128_delegate,
            state: &mut self.state,
        };
        let next_bool_delegate = &mut self.next_bool_delegate;
        let r = next_bool_delegate(surrogate, p);
        self.state.next_bool_invocations += 1;
        r
    }

    fn next_lim_u16(&mut self, lim: u16) -> u16 {
        self.next_lim_u128(u128::from(lim)) as u16
    }

    fn next_lim_u32(&mut self, lim: u32) -> u32 {
        self.next_lim_u128(u128::from(lim)) as u32
    }

    fn next_lim_u64(&mut self, lim: u64) -> u64 {
        self.next_lim_u128(u128::from(lim)) as u64
    }

    /// Delegates to the underlying closure and increments the `state.next_lim_u128_invocations` counter
    /// _after_ the closure returns.
    fn next_lim_u128(&mut self, lim: u128) -> u128 {
        let surrogate = Surrogate {
            next_u128_delegate: &mut self.next_u128_delegate,
            state: &mut self.state,
        };
        let next_lim_delegate = &mut self.next_lim_u128_delegate;
        let r = next_lim_delegate(surrogate, lim);
        self.state.next_lim_u128_invocations += 1;
        r
    }
}

/// A pre-canned delegate that counts in the given range, wrapping around when it reaches
/// the end.
///
/// # Examples
/// ```
/// use tinyrand::Rand;
/// use tinyrand_alloc::{Mock, counter};
///
/// let mut mock = Mock::default().with_next_u128(counter(5..8));
/// assert_eq!(5, mock.next_u64());
/// assert_eq!(6, mock.next_u64());
/// assert_eq!(7, mock.next_u64());
/// assert_eq!(5, mock.next_u64());
/// ```
pub fn counter(range: Range<u128>) -> impl FnMut(&State) -> u128 {
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
/// use tinyrand::Rand;
/// use tinyrand_alloc::{Mock, fixed};
///
/// let mut mock = Mock::default().with_next_u128(fixed(42));
/// assert_eq!(42, mock.next_u64());
/// assert_eq!(42, mock.next_u64());
/// ```
pub fn fixed(val: u128) -> impl FnMut(&State) -> u128 {
    move |_| val
}

/// A pre-canned delegate that parrots the value in the given cell.
///
/// # Examples
/// ```
/// use std::cell::RefCell;
/// use tinyrand::{Rand, RefCellExt};
/// use tinyrand_alloc::{Mock, echo};
///
/// let cell = RefCell::default();
/// let mut mock = Mock::default().with_next_u128(echo(&cell));
/// assert_eq!(0, mock.next_u64());
/// cell.set(42);
/// assert_eq!(42, mock.next_u64());
/// ```
pub fn echo(cell: &RefCell<u128>) -> impl FnMut(&State) -> u128 + '_ {
    move |_| *cell.borrow()
}

#[cfg(test)]
mod tests;
