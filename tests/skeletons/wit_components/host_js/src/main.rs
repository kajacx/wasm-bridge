const PLUGIN_PATH: &'static str = "../../plugin/target/wasm32-unknown-unknown/debug/out-dir/";

mod host;

#[wasm_bindgen_test::wasm_bindgen_test]
fn main() {
    host::run_test(PLUGIN_PATH.as_bytes()).expect("host_js test should pass")
}
