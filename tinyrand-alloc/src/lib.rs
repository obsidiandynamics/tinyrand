//! Extensions for using `tinyrand` with `alloc`.

#![no_std]

extern crate alloc;

pub mod mock;

pub use mock::*;