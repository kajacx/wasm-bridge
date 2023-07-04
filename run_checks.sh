#!/usr/bin/sh
set -e

RUSTFLAGS="-D warnings" cargo check --all-features
RUSTFLAGS="-D warnings" cargo check --all-features --target=wasm32-unknown-unknown

cargo clippy --all-features -- -D clippy::all
cargo clippy --all-features --target=wasm32-unknown-unknown -- -D clippy::all

cd tests
./run_all_tests.sh
