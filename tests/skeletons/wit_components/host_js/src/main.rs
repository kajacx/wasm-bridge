const GUEST_BYTES: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-unknown-unknown/debug/component.wasm");

const UNIVERSAL: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-unknown-unknown/debug/universal.zip");

mod host;

#[wasm_bindgen_test::wasm_bindgen_test]
fn main() {
    host::run_test(GUEST_BYTES, UNIVERSAL).expect("host_js test should pass")
}
