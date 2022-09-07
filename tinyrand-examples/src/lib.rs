//! Examples for `tinyrand`.
//!
//! See the `examples` directory. It is an otherwise empty lib crate.

use std::cell::RefCell;

/// Used by examples.
#[derive(Default)]
pub struct SomeSpecialCondition {
    count: RefCell<u32>
}

impl SomeSpecialCondition {
    pub fn has_happened(&self) -> bool {
        if *self.count.borrow() == 10 {
            true
        } else {
            *self.count.borrow_mut() += 1;
            false
        }
    }
}

/// Used by the examples in `../../README.md`.
pub fn get_seed_from_somewhere() -> u64 {
    42_242_424_242
}

#[doc = include_str!("../../README.md")]
#[cfg(doc)]
fn readme() {}