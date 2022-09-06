//! Advanced mock RNG for testing.

use alloc::boxed::Box;
use alloc::rc::Rc;
use tinyrand::{Probability, Rand};
use tinyrand::mock::{fixed, U64Cell};

/// Mock delegate for [`Rand::next_u64`].
pub type NextU64 = Box<dyn FnMut(&State) -> u64>;

/// Mock delegate for [`Rand::next_bool`].
pub type NextBool = Box<dyn FnMut(Surrogate, Probability) -> bool>;

/// Mock delegate for [`Rand::next_lim_u128`].
pub type NextLim = Box<dyn FnMut(Surrogate, u128) -> u128>;

/// Mock invocation state.
#[derive(Default)]
pub struct State {
    next_u64_invocations: u64,
    next_bool_invocations: u64,
    next_lim_u128_invocations: u64,
}

impl State {
    /// Obtains the number of invocations of the [`Rand::next_u64`] method.
    pub fn next_u64_invocations(&self) -> u64 {
        self.next_u64_invocations
    }

    /// Obtains the number of invocations of the [`Rand::next_bool`] method.
    pub fn next_bool_invocations(&self) -> u64 {
        self.next_bool_invocations
    }

    /// Obtains the number of invocations of the [`Rand::next_lim_u128`] method.
    pub fn next_lim_u128_invocations(&self) -> u64 {
        self.next_bool_invocations
    }
}

/// Encapsulates the state of the mock and a reference to the [`NextU64`] delegate, so
/// that it can be invoked from inside the mock by, for example, another delegate.
pub struct Surrogate<'a> {
    state: &'a mut State,
    next_u64_delegate: &'a mut NextU64
}

impl<'a> Surrogate<'a> {
    /// Obtains the mock state.
    pub fn state(&self) -> &State {
        self.state
    }
}

impl<'a> Rand for Surrogate<'a> {
    fn next_u64(&mut self) -> u64 {
        let r = (self.next_u64_delegate)(self.state);
        self.state.next_u64_invocations += 1;
        r
    }
}

// Mock RNG, containing invocation state and delegate closures.
pub struct AdvMock {
    state: State,
    next_u64_delegate: NextU64,
    next_bool_delegate: NextBool,
    next_lim_u128_delegate: NextLim,
}

impl Default for AdvMock {
    fn default() -> Self {
        Self {
            state: State::default(),
            next_u64_delegate: Box::new(fixed(0)),
            next_bool_delegate: Box::new(|mut surrogate, p| {
                Rand::next_bool(&mut surrogate, p)
            }),
            next_lim_u128_delegate: Box::new(|mut surrogate, lim| {
                Rand::next_lim_u128(&mut surrogate, lim)
            })
        }
    }
}

impl AdvMock {
    /// Assigns a [`Rand::next_u64`] delegate to the mock. I.e., when the [`Rand::next_u64`]
    /// method is invoked on the mock, it will delegate to the given closure.
    #[must_use]
    pub fn with_next_u64(mut self, delegate: impl FnMut(&State) -> u64 + 'static) -> Self {
        self.next_u64_delegate = Box::new(delegate);
        self
    }

    /// Assigns a [`Rand::next_bool`] delegate to the mock. I.e., when the [`Rand::next_bool`]
    /// method is invoked on the mock, it will delegate to the given closure.
    #[must_use]
    pub fn with_next_bool(mut self, delegate: impl FnMut(Surrogate, Probability) -> bool + 'static) -> Self {
        self.next_bool_delegate = Box::new(delegate);
        self
    }

    /// Assigns a [`Rand::next_lim_u128`] delegate to the mock. I.e., when the [`Rand::next_lim_u128`]
    /// method is invoked on the mock, it will delegate to the given closure.
    #[must_use]
    pub fn with_next_lim_u128(mut self, delegate: impl FnMut(Surrogate, u128) -> u128 + 'static) -> Self {
        self.next_lim_u128_delegate = Box::new(delegate);
        self
    }

    /// Obtains the underlying mock state.
    pub fn state(&self) -> &State {
        &self.state
    }
}

impl Rand for AdvMock {
    /// Delegates to the underlying closure and increments the `state.next_u64_invocations` counter
    /// _after_ the closure returns.
    fn next_u64(&mut self) -> u64 {
        let next_u64_delegate = &mut self.next_u64_delegate;
        let r = next_u64_delegate(&self.state);
        self.state.next_u64_invocations += 1;
        r
    }

    /// Delegates to the underlying closure and increments the `state.next_bool_invocations` counter
    /// _after_ the closure returns.
    fn next_bool(&mut self, p: Probability) -> bool {
        let surrogate = Surrogate {
            next_u64_delegate: &mut self.next_u64_delegate,
            state: &mut self.state
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
            next_u64_delegate: &mut self.next_u64_delegate,
            state: &mut self.state
        };
        let next_lim_delegate = &mut self.next_lim_u128_delegate;
        let r = next_lim_delegate(surrogate, lim);
        self.state.next_lim_u128_invocations += 1;
        r
    }
}

/// A pre-canned delegate that parrots the value on the heap.
///
/// # Examples
/// ```
/// use std::rc::Rc;
/// use tinyrand::{Mock, Rand};
/// use tinyrand::mock::{counter, echo, U64Cell};
/// use tinyrand_alloc::echo_heap;
/// let cell = Rc::new(U64Cell::default());
/// let mut mock = Mock::new(echo_heap(cell.clone()));
/// assert_eq!(0, mock.next_u64());
/// cell.set(42);
/// assert_eq!(42, mock.next_u64());
/// ```
pub fn echo_heap<S>(cell: Rc<U64Cell>) -> impl FnMut(&S) -> u64 {
    move |_| *cell.borrow()
}

#[cfg(test)]
mod tests;