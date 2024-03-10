const GUEST_BYTES: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-unknown-unknown/release/no_bindgen_guest.wasm");

mod host;

#[wasm_bindgen_test::wasm_bindgen_test]
async fn main() {
    host::run_test(GUEST_BYTES).await.expect("host_js test should pass")
}

#[allow(dead_code)]
fn disable_sync_wasm_functions() {
    // Disable the synchronous variants of WebAssembly.compile and WebAssembly.instantiate,
    // so that this test properly checks that we are actually using the asynchronous ones.
    wasm_bridge::js_sys::eval("WebAssembly.Module = 'Disabled'; WebAssembly.Instance = 'Disabled';").unwrap();
}
