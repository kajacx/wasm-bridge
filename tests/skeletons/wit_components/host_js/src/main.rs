const PLUGIN_ZIP: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/out-dir.zip");

const UNIVERSAL: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/universal.zip");

mod host;

#[wasm_bindgen_test::wasm_bindgen_test]
fn main() {
    host::run_test(PLUGIN_ZIP, UNIVERSAL).expect("host_js test should pass")
}
