# Avoiding the universal component

First, read [My first component](./my_first_component.md) to understand how to get started.

## Full minimal example

Full example of not using universal mode can be found [here](https://github.com/kajacx/wasm-playground/tree/wasm-bridge-02).

## Summary

In universal mode, the zipped component contains both the original wasm component, and the jco transpiled modules.

This makes the file about twice as large as it needs to be, but it's instead possible to separate the two components.

However, you will have to **pass different bytes** to `Component::new()` on desktop and on the web.

## Component generation

When running `wasm-bridge-cli`, you can omit the `--universal` flag and it's argument.

This creates a zip file with only the jco-transpiled modules that will work on the web. This zip will however **not work on sys** (desktop).

## Desktop runtime optimization

On sys, you can pass the original `component.wasm` file to either `Component::new` or `wasm_bridge::component::new_universal_component`.

However, you can use the `component-model-no-universal` feature instead of `component-model`, which disables the `new_universal_component` function
and removes the `zip` dependency.

You will have to use `Component::new`, and pass the `component.zip` file on the web instead.
