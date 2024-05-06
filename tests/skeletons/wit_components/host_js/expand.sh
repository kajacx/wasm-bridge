#!/usr/bin/sh
set -e

cargo expand --tests --target wasm32-unknown-unknown > expanded.rs
