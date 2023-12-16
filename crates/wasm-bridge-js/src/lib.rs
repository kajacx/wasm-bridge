// mod engine;
// pub use engine::*;

// mod store;
// pub use store::*;

// mod no_bindgen;
// pub use no_bindgen::*;

// mod conversions;
// pub use conversions::*;

// mod caller;
// pub use caller::*;

// mod config;
// pub use config::*;

// mod context;
// pub use context::*;

// pub type Error = anyhow::Error;
// pub type Result<T, E = Error> = anyhow::Result<T, E>;

// pub mod helpers;

// #[cfg(feature = "component-model")]
// pub mod component;

// #[cfg(feature = "wasi")]
// pub mod wasi;

// #[cfg(feature = "async")]
// pub use wasm_bridge_macros::async_trait;

// pub use js_sys;
// pub use wasm_bindgen;

use wasm_bindgen::{closure::Closure, convert::*, JsValue};

fn is_from_wasm_abi<T: FromWasmAbi>() {}
fn is_return_wasm_abi<T: ReturnWasmAbi>() {}

fn test_it<R: IntoWasmAbi>() {
    is_return_wasm_abi::<Result<R, JsValue>>();
}

fn make() {
    is_from_wasm_abi::<u32>();
    is_from_wasm_abi::<String>();
    is_from_wasm_abi::<Option<u32>>();
    //is_from_wasm_abi::<()>(); // no
    //is_from_wasm_abi::<Result<u32, String>>(); // no
    // is_from_wasm_abi::<(String, u8)>(); // no
    let a = Closure::<dyn Fn() -> i32>::new(|| 5);
}
