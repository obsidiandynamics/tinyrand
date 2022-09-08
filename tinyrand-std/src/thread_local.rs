//! Thread-local [`Rand`].

use crate::ClockSeed;
use core::cell::RefCell;
use std::rc::Rc;
use tinyrand::{Rand, Seeded, StdRand};

thread_local! {
    static THREAD_LOCAL_RAND: Rc<RefCell<StdRand>> = Rc::new(RefCell::new(StdRand::seed(ClockSeed::default().next_u64())));
}

/// A seeded, thread-local [`Rand`] instance.
#[allow(clippy::module_name_repetitions)]
pub struct ThreadLocalRand(Rc<RefCell<StdRand>>);

impl Rand for ThreadLocalRand {
    #[inline(always)]
    fn next_u16(&mut self) -> u16 {
        self.0.borrow_mut().next_u16()
    }

    #[inline(always)]
    fn next_u32(&mut self) -> u32 {
        self.0.borrow_mut().next_u32()
    }

    #[inline(always)]
    fn next_u64(&mut self) -> u64 {
        self.0.borrow_mut().next_u64()
    }

    #[inline(always)]
    fn next_u128(&mut self) -> u128 {
        self.0.borrow_mut().next_u128()
    }
}

/// Obtains a seeded, thread-local [`Rand`] instance.
pub fn thread_rand() -> ThreadLocalRand {
    let cell = THREAD_LOCAL_RAND.with(std::clone::Clone::clone);
    ThreadLocalRand(cell)
}

#[cfg(test)]
mod tests;
