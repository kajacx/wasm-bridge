# wasm-bridge

## Goals

The goal of this crate is to "run wasmtime on the web".

Since wasmtime cannot actually *run* on the web, the goal is to **provide a unified API** for both sys (desktop) and js (web) runtimes.

The provided API will be identital to wasmtime's API, so read wasmtime's documentation on how to use this crate.

## How to install

This crate is not yet ready for public use. You can clone the repo or includide it as a git dependency at your own risk.

## Switching from `wasmtime`

If you are using `wasmtime`, using this crate should be as simple as replacing the `wasmtime` dependency with `wasm-bridge` dependency (well, once a usable version of `wasm-bridge` is released).

## Using `component-model`

Work on supporting the component model has only just begun, but it will hopefully be possible to use the component model with `wasm-bridge` in the future.

## Implemented features
- Load a module from bytes
- Instantiate a module with an empty import object
- Get (typed) exported function and call it
- Add imported functions to a linker
- Instantiate a module with a linker to use imported functions
- Compile a module from the wat format
- Multivalue returns from exported and imported functions
- Supported value types: `i32`, `i64`, `f32`, `f64`

See the [`no_bindgen`](tests/no_bindgen) folder for supported example usages.
