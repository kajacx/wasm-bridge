#[cfg(not(target_arch = "wasm32"))]
mod sys;

#[cfg(not(target_arch = "wasm32"))]
pub use sys::*;

#[cfg(target_arch = "wasm32")]
pub use wasm_bridge_js::*;

#[test]
fn test() {
    panic!("To test `wasm-bridge`, run the `run_all_tests.sh` script from the `tests` folder.");
}
