# wasm-bridge-cli

This is a helper crate for `wasm-bridge`, see [crates.io](https://crates.io/crates/wasm-bridge) or the [GitHub repo](https://github.com/kajacx/wasm-bridge).

## Install

Install with `cargo install wasm-bridge-cli`

## Usage

After you get the "out-dir" from the `jco transpile` command, for example:

`jco transpile component.wasm --instantiation -o out-dir`

You can then use the `wasm-bridge-cli` command and pass it this directory, for example:

`wasm-bridge-cli out-dir component-web.zip`

Now you can load the bytes of `component-web.zip` into `wasm_bridge::component::Component::new()` on the web.

Note that you still need to pass the original `component.wasm` into `wasm_bridge::component::Component::new()` on sys.

See [Component model](/component_model.md) for the full workflow.
