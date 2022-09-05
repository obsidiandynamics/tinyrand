#!/bin/sh
cargo clippy -- -D clippy::pedantic -A clippy::must-use-candidate -A clippy::struct-excessive-bools -A clippy::single-match-else -A clippy::inline-always -A clippy::cast-possible-truncation
