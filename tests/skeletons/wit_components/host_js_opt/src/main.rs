const GUEST_BYTES: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-unknown-unknown/release/wit_components_guest.wasm");

mod host;

#[wasm_bindgen_test::wasm_bindgen_test]
fn main() {
    host::run_test(GUEST_BYTES).expect("host_js test should pass")
}
