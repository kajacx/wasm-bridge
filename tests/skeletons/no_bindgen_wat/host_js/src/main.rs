const PLUGIN_WAT: &'static [u8] = include_bytes!("../../plugin/plugin.wat");

mod host;

#[wasm_bindgen_test::wasm_bindgen_test]
fn main() {
    host::run_test(PLUGIN_WAT).expect("host_js test should pass")
}
