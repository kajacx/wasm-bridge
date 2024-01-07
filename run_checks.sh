#!/usr/bin/sh
set -e

# Unit tests
cd crates/wasm-bridge-js
wasm-pack test --node -- --all-features
cd ../..

cd crates/wasm-bridge-wasi
wasm-pack test --node -- --all-features
cd ../..

# Warnigns
cargo check --target wasm32-unknown-unknown --all-features
cargo clippy --target wasm32-unknown-unknown --all-features -- -D clippy::all

# Formatting
cargo fmt --all -- --check

# Acceptance tests
cd tests
./run_all_tests.sh
