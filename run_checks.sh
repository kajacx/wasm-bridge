#!/usr/bin/sh
set -e

# RUSTFLAGS="-D warnings" cargo check --all-features
RUSTFLAGS="-D warnings" cargo check --all-features --target=wasm32-unknown-unknown

# RUSTFLAGS="-D warnings" cargo clippy --all-features -- -D clippy::all
RUSTFLAGS="-D warnings" cargo clippy --all-features --target=wasm32-unknown-unknown -- -D clippy::all

cargo fmt --all -- --check

cd tests
./run_all_tests.sh
