## Test examples

Here are use case examples on using wasm-bridge with wasmtime's component model. Unlike the rest of the documentation, there examples are guaranteed to be up to date, because running them is a part of the CI pipeline.

## Project setup

Setup is a little bit convoluted (to enable sharing built packages between test) and can be seen in the [run test script](../skeletons/wit_components/run_test.sh), but here are the basic steps:

1. Copy the [guest](../skeletons/wit_components/guest/) folder somewhere.
2. Copy the `guest.rs` file from a test to the `src` guest folder and rename it to `lib.rs`.
3. In the guest folder, run `cargo component build --target wasm32-unknown-unknown`.
4. Copy the [host](../skeletons/wit_components/host_sys/) folder somewhere.
5. Replace the "path" dependency in the host `Cargo.toml` with a current version dependency.
5. Copy the `host.rs` file from a test to the `src` host folder.
6. Make sure the the path in the `lib.rs` host file points to the built component (might need to replace `release` with `debug`).
7. Run with `cargo test` in the host folder to verify that it is working.
