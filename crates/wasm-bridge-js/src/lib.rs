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

#[cfg(all(feature = "component-model", not(feature = "direct-bytes")))]
pub mod component;

#[cfg(all(feature = "component-model", feature = "direct-bytes"))]
pub mod component_direct;
#[cfg(all(feature = "component-model", feature = "direct-bytes"))]
pub use component_direct as component;

#[cfg(feature = "async")]
pub use wasm_bridge_macros::async_trait;

#[cfg(feature = "direct-bytes")]
pub mod direct_bytes;
#[cfg(feature = "direct-bytes")]
pub use direct_bytes::next_multiple_of;
#[cfg(feature = "direct-bytes")]
pub use direct_bytes::usize_max;

pub use js_sys;
pub use wasm_bindgen;
