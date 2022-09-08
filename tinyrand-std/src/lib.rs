//! Extensions for using `tinyrand` with `stdlib`.

pub mod clock_seed;
pub mod thread_local;

pub use clock_seed::ClockSeed;
pub use thread_local::ThreadLocalRand;
pub use thread_local::thread_rand;