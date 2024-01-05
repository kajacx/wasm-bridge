const GUEST_BYTES: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-unknown-unknown/release/no_bindgen_guest.wasm");

mod host;

#[wasm_bindgen_test::wasm_bindgen_test]
async fn main() {
    host::run_test(GUEST_BYTES).await.expect("host_js test should pass")
}
