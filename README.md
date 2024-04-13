# wasm-bridge

<div align="center">
  <p>
    <img src="/wasm-bridge-banner.png" alt="image banner" width="600" />
  </p>

  <p>
    <a href="https://www.flaticon.com/" title="Icons from flaticon.com">
      <img src="https://img.shields.io/badge/Icons_from-Flaticon-teal" alt="Icons from flaticon.com">
    </a>
    <img src="https://img.shields.io/badge/%E2%9C%85-No_unsafe-green" alt="No unsafe">
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

In general, you would use this crate the same way wou would use `wasmtime`, and just replace `wasmtime` with `wasm_bride`.

Alternatively, refer to this handy table:

| Use case | 📝 Text tutorial | ▶️ Video tutorial | 🧾 Full example | 📑 Test cases |
| --- | --- | --- | --- | --- |
| **No bindgen** | [Read here](/docs/wasm_modules.md) | Coming soon | [See here](https://github.com/kajacx/wasm-tutorials/tree/wbtutor-wasm-modules) | [Explore here](/tests/no_bindgen/) |
| **Component model** | [Read here](/docs/wit_components.md) | Coming soon | Coming soon | [Explore here](/tests/wit_components/) |
| **Component model with WASI** | Coming soon | Coming soon | Coming soon | [Explore here](/tests/wasi_components/) |


## Versions

| `wasm-bridge` | `wasmtime` | `wit-bindgen` | `cargo-component` |
| ---           | ---        | ---           | ---               |
| `0.4.x`       | `19.0`     | `0.24.0`      | `0.10.1`          |
| `0.3.x`       | `15.0`     | `0.15.0`      | `0.5.0`           |
| `0.2.x`       | `11.0`     | `0.8.0`       | -                 |
| `0.1.x`       | `10.0`     | -             | -                 |


## License

The source code of `wasm-bride` is licensed under MIT, but there are portions that are copied from other projects,
and may come with a different license.

Here is a full list of these exceptions:

- The [`wasmtime-component-macro`](/crates/wasm-bridge-macros/src/original) crate "fork".
