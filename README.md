# wasm-bridge

<div align="center">
  <p>
    <img src="/wasm-bridge-banner.png" alt="image banner" width="600" />
  </p>

  <p>
    <a href="https://www.flaticon.com/" title="Icons from flaticon.com">
      <img src="https://img.shields.io/badge/Icons_from-Flaticon-green" alt="Icons from flaticon.com">
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

In general, you would use this crate the same way wou would use `wasmtime`, and just replace `wasmtime` with `wasm_bride`.

Alternatively, you can take a look at these examples:

 - **No bindgen:** Read the [text tutorial](/docs/wasm_modules.md), watch the video showcase, or see a [full example](https://github.com/kajacx/wasm-tutorials/tree/wbtutor-wasm-modules).
 - **Component model:** Read the [text tutorial](/docs/wit_components.md), watch the video showcase, or see a full example.
 - **Component model with WASI:** Read the [text tutorial](/docs/wasi_compnents.md), watch the video showcase, or see a full example.


## License

The source code of `wasm-bride` is licensed under MIT, but there are portions that are copied from other projects,
and may come with a different license.

Here is a full list of these exceptions:

- The [`wasmtime-component-macro`](/crates/wasm-bridge-macros/src/original) crate "fork".
