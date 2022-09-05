#!/bin/sh
set -e

# continuously test the system, using alternating values for the release profile and the number of test threads
while [ true ]; do
  echo "[$(date)] Release + single-threaded"
  cargo test --profile release -- --test-threads 1
  echo "[$(date)] Debug + single-threaded"
  cargo test --profile dev -- --test-threads 1
  echo "[$(date)] Release + multi-threaded"
  cargo test --profile release
  echo "[$(date)] Debug + multi-threaded"
  cargo test --profile dev
done