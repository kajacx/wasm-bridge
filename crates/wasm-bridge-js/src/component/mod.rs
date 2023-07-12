#[allow(clippy::module_inception)]
mod component;
pub use component::*;

mod linker;
pub use linker::*;

mod instance;
pub use instance::*;

mod exports;
pub use exports::*;

mod func;
pub use func::*;

mod typed_func;
pub use typed_func::*;

mod make_closure;
pub use make_closure::*;

pub use wasm_bridge_macros::bindgen_js as bindgen;

pub use wasm_bridge_macros::FromJsValue;
pub use wasm_bridge_macros::ToJsValue;

pub mod __internal {
    pub use anyhow;
}
