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

pub use wasm_bridge_macros::*;

pub mod __internal {
    pub use anyhow;
}
