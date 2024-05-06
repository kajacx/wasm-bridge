#!/usr/bin/sh
set -e

cargo expand --lib --tests > expanded.rs
