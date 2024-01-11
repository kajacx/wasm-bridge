mod engine;
pub use engine::*;

mod store;
pub use store::*;

mod no_bindgen;
pub use no_bindgen::*;

mod conversions;
pub use conversions::*;

mod caller;
pub use caller::*;

mod config;
pub use config::*;

mod context;
pub use context::*;

pub type Error = anyhow::Error;
pub type Result<T, E = Error> = anyhow::Result<T, E>;
pub use anyhow::bail;
pub use anyhow::Context;

pub mod helpers;

#[cfg(all(feature = "component-model", not(feature = "optimize")))]
pub mod component;

#[cfg(all(feature = "component-model", feature = "optimize"))]
pub mod component_optimize;
#[cfg(all(feature = "component-model", feature = "optimize"))]
pub use component_optimize as component;

#[cfg(feature = "component-model")]
pub mod direct;
#[cfg(feature = "component-model")]
pub use direct::next_multiple_of;
#[cfg(feature = "component-model")]
pub use direct::usize_max;

#[cfg(feature = "async")]
pub use wasm_bridge_macros::async_trait;
// pub use async_trait::async_trait;

pub use js_sys;
pub use wasm_bindgen;
