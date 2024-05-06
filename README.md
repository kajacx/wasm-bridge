# wasm-bridge

<div align="center">
  <p>
    <img src="/wasm-bridge-banner.png" alt="image banner" width="600" />
  </p>

  <p>
    <a href="https://www.flaticon.com/" title="Icons from flaticon.com">
      <img src="https://img.shields.io/badge/Icons_from-Flaticon-teal" alt="Icons from flaticon.com">
    </a>
    <a href="https://crates.io/crates/wasm-bridge" title="View on crates.io">
      <img src="https://img.shields.io/badge/View_on-crates.io-blue" alt="View on crates.io">
    </a>
    <a href="https://discord.gg/7fk5Uk6Eqr" title="Join the Discord server">
      <img src="https://img.shields.io/discord/1125842158914646080?logo=discord&label=Join" alt="Join the Discord server">
    </a>
  </p>
</div>


## Goals

The goal of this crate is to "run wasmtime on the web", that means **providing a unified API** for desktop and web runtimes.

With `wasm-bridge`, you can write a *single source code in Rust* that you would normally write when using wasmtime,
but it works on desktop as well as on the web.


## How do I use this?

In general, you would use this crate the same way wou would use `wasmtime`, and just replace `wasmtime` with `wasm_bridge`.

Alternatively, refer to this handy table:

| Use case | 📝 Text tutorial | ▶️ Video tutorial | 🧾 Full example | 📑 Test cases |
| --- | --- | --- | --- | --- |
| **No bindgen** | [Read here](/docs/wasm_modules.md) | Coming soon | [See here](https://github.com/kajacx/wasm-tutorials/tree/wbtutor-wasm-modules) | [Explore here](/tests/no_bindgen/) |
| **Component model** | [Read here](/docs/wit_components.md) | Coming soon | Coming soon | [Explore here](/tests/wit_components/) |
| **Component model with WASI** | Coming soon | Coming soon | Coming soon | [Explore here](/tests/wasi_components/) |


## Versions

| `wasm-bridge` | `wasmtime` | `wit-bindgen` | `cargo-component` |
| ---           | ---        | ---           | ---               |
| `0.5.x`       | `20.0`     | `0.24.0`      | `0.11.0`          |
| `0.4.x`       | `19.0`     | `0.24.0`      | `0.10.1`          |
| `0.3.x`       | `15.0`     | `0.15.0`      | `0.5.0`           |
| `0.2.x`       | `11.0`     | `0.8.0`       | -                 |
| `0.1.x`       | `10.0`     | -             | -                 |


## Alternatives

There are other options for loading and executing WASM modules on the desktop and on the web in RUST: `wasmer` can run on the web with the `js` feature flag, `wasmi` is an interpreter so it has no problem running on the web, and `wasm_component_layer` provides a unified API that can be "backed" by a number of "backends".

| Crate | Short description | Component model | Wasi support |
| ---   | ---               | ---             | ---          |
| `wasm-bridge` | Re-exports `wasmtime` on sys, `js-sys` impl on web. | Yes, but no resources. | Partially yes. |
| [`wasmer`](https://github.com/wasmerio/wasmer) | Native impl in sys, `js-sys` impl on web. | They have [`wai` bindgen](https://github.com/wasmerio/wai). | [Yes](https://crates.io/crates/wasmer-wasi). |
| [`wasmi`](https://github.com/wasmi-labs/wasmi) | Lightweight WASM interpreter, run anywhere. | [Planned](https://github.com/wasmi-labs/wasmi/issues/897). | Experimental [`wasmi_wasi`](https://github.com/wasmi-labs/wasmi/tree/master/crates/wasi) crate. |
| [`wasm_runtime_layer`](https://github.com/DouglasDwyer/wasm_runtime_layer) | Thin wrapper around `wasmtime` or `wasmi`, `js-sys` impl on web. | [`wasm_component_layer`](https://github.com/DouglasDwyer/wasm_component_layer), but no bindgen. | Not to my knowledge. |

## License

The source code of `wasm-bride` is licensed under MIT, but there are portions that are copied from other projects,
and may come with a different license.

Here is a full list of these exceptions:

- The [`wasmtime-component-macro`](/crates/wasm-bridge-macros/src/original) crate "fork".
