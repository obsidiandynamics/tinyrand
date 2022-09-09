//! Extensions for using `tinyrand` with `stdlib`.

pub mod clock_seed;
pub mod thread_local;

pub use clock_seed::ClockSeed;
pub use thread_local::ThreadLocalRand;
pub use thread_local::thread_rand;

#[cfg(test)]
mod tests {
    /// All this does is print the pointer width. Useful for determining the `usize` width
    /// of the current platform.
    #[test]
    fn pointer_width() {
        println!("{}-bit", num_bits::<usize>());
    }

    const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }
}