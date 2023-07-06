const PLUGIN_ZIP: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/out-dir.zip");

mod host;

#[wasm_bindgen_test::wasm_bindgen_test]
fn main() {
    host::run_test(PLUGIN_ZIP).expect("host_js test should pass")
}
