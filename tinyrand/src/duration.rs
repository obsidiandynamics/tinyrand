//! Extensions for generating random `Duration`s.

use core::ops::Range;
use core::time::Duration;
use crate::{Rand, RandLim, RandRange};

impl<R: Rand> RandRange<Duration> for R {
    #[inline(always)]
    fn next_range(&mut self, range: Range<Duration>) -> Duration {
        assert!(!range.is_empty(), "empty range");
        let span = (range.end - range.start).as_nanos();
        let random = self.next_lim(span);
        range.start + from_nanos(random)
    }
}

/// [`Duration::from_nanos`] has limited range, which [was not reverted post-stabilisation](https://github.com/rust-lang/rust/issues/51107).
/// This function permits the creation of a [`Duration]` from a `u128`, making it consistent with
/// [`Duration::as_nanos`].
#[inline(always)]
pub const fn from_nanos(nanos: u128) -> Duration {
    const NANOS_PER_SEC: u128 = 1_000_000_000;
    let secs = (nanos / NANOS_PER_SEC) as u64;
    let nanos = (nanos % NANOS_PER_SEC) as u32;
    Duration::new(secs, nanos)
}

#[cfg(test)]
mod tests;