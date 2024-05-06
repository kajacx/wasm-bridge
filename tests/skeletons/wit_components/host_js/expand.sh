#!/usr/bin/sh
set -e

cargo expand --target wasm32-unknown-unknown > expanded.rs
