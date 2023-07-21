#!/usr/bin/sh
set -e

# Unit tests
cd crates/wasm-bridge-js
wasm-pack test --node -- --all-features
cd ../..

# Warnigns
cargo check --all-features
cargo clippy --all-features -- -D clippy::all

# Formatting
cargo fmt --all -- --check

# Acceptance tests
cd tests
./run_all_tests.sh
