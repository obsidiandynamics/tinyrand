//! Advanced mock RNG for testing.

use alloc::boxed::Box;
use core::cell::{Ref, RefCell, RefMut};
use core::ops::{Range};
use tinyrand::{Probability, Rand};

/// Mock delegate for [`Rand::next_u64`].
pub type NextU64 = Box<dyn FnMut(&State) -> u64>;

/// Mock delegate for [`Rand::next_bool`].
pub type NextBool = Box<dyn FnMut(Surrogate, Probability) -> bool>;

/// Mock state.
#[derive(Default)]
pub struct State {
    next_u64_invocations: u64,
    next_bool_invocations: u64,
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
}

pub struct Surrogate<'a> {
    state: &'a State,
    next_u64_delegate: &'a mut NextU64
}

impl<'a> Surrogate<'a> {
    pub fn state(&self) -> &State {
        self.state
    }
}

impl<'a> Rand for Surrogate<'a> {
    fn next_u64(&mut self) -> u64 {
        (self.next_u64_delegate)(self.state)
    }
}

// Mock RNG, initialised with a delegate closure.
pub struct AdvMock {
    state: State,
    next_u64_delegate: NextU64,
    next_bool_delegate: NextBool,
}

impl Default for AdvMock {
    fn default() -> Self {
        Self {
            state: State::default(),
            next_u64_delegate: Box::new(fixed(0)),
            next_bool_delegate: Box::new(|mut surrogate, p| {
                Rand::next_bool(&mut surrogate, p)
            })
        }
    }
}

impl AdvMock {
    #[must_use]
    pub fn with_next_u64(mut self, delegate: impl FnMut(&State) -> u64 + 'static) -> Self {
        self.next_u64_delegate = Box::new(delegate);
        self
    }

    #[must_use]
    pub fn with_next_bool(mut self, delegate: impl FnMut(Surrogate, Probability) -> bool + 'static) -> Self {
        self.next_bool_delegate = Box::new(delegate);
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

    fn next_bool(&mut self, p: Probability) -> bool {
        let surrogate = Surrogate {
            next_u64_delegate: &mut self.next_u64_delegate,
            state: &self.state
        };
        let next_bool_delegate = &mut self.next_bool_delegate;
        let r = next_bool_delegate(surrogate, p);
        self.state.next_bool_invocations += 1;
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
/// use tinyrand::mock::fixed;
/// let mut mock = Mock::new(fixed(42));
/// assert_eq!(42, mock.next_u64());
/// assert_eq!(42, mock.next_u64());
/// ```
pub fn fixed(val: u64) -> impl FnMut(&State) -> u64 {
    move |_| val
}

/// An internally mutable `u64`.
#[derive(Debug)]
pub struct U64Cell(RefCell<u64>);

impl Default for U64Cell {
    fn default() -> Self {
        Self::new(0)
    }
}

impl U64Cell {
    /// Creates a new cell.
    pub fn new(initial: u64) -> Self {
        Self(RefCell::new(initial))
    }

    /// Immutably borrows the contained value.
    ///
    /// # Panics
    /// If the value is mutably borrowed elsewhere.
    pub fn borrow(&self) -> Ref<u64> {
        self.0.borrow()
    }

    /// Mutably borrows the contained value.
    ///
    /// # Panics
    /// If the value is mutably borrowed elsewhere.
    pub fn borrow_mut(&self) -> RefMut<u64> {
        self.0.borrow_mut()
    }

    /// Obtains the current value.
    ///
    /// # Panics
    /// If the value is mutably borrowed elsewhere.
    pub fn get(&self) -> u64 {
        *self.borrow()
    }

    /// Assigns a new value.
    ///
    /// # Panics
    /// If the value is mutably borrowed elsewhere.
    pub fn set(&self, val: u64) {
        *self.borrow_mut() = val;
    }
}

/// A pre-canned delegate that parrots the value contained in the given cell.
///
/// # Examples
/// ```
/// use tinyrand::{Mock, Rand};
/// use tinyrand::mock::{counter, echo, U64Cell};
/// let cell = U64Cell::default();
/// let mut mock = Mock::new(echo(&cell));
/// assert_eq!(0, mock.next_u64());
/// cell.set(42);
/// assert_eq!(42, mock.next_u64());
/// ```
pub fn echo(cell: &U64Cell) -> impl FnMut(&State) -> u64 + '_ {
    |_| *cell.borrow()
}

#[cfg(test)]
mod tests;