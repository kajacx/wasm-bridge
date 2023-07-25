mod engine;
pub use engine::*;

mod instance;
pub use instance::*;

mod module;
pub use module::*;

mod store;
pub use store::*;

mod typed_func;
pub use typed_func::*;

mod conversions;
pub use conversions::*;

mod linker;
pub use linker::*;

mod caller;
pub use caller::*;

mod config;
pub use config::*;

mod context;
pub use context::*;

mod val;
pub use val::*;

mod types;
pub use types::*;

pub type Error = anyhow::Error;
pub type Result<T, E = Error> = anyhow::Result<T, E>;

pub mod helpers;

#[cfg(feature = "component-model")]
pub mod component;

#[cfg(feature = "wasi")]
pub mod wasi;

#[cfg(feature = "async")]
pub use wasm_bridge_macros::async_trait;

pub use js_sys;
pub use wasm_bindgen;
