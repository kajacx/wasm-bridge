#!/usr/bin/sh
set -e

RUSTFLAGS="-D warnings" cargo check --all-features

RUSTFLAGS="-D warnings" cargo clippy --all-features -- -D clippy::all

cargo fmt --all -- --check

cd tests
./run_all_tests.sh
