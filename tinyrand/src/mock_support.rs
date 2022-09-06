//! Supporting capabilities for mocking.

use core::cell::RefCell;

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