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

mod helpers;

// pub use wasm_bridge_macros::*;
pub use wasmtime_component_macro::*;

pub mod __internal {
    pub use anyhow;
}
