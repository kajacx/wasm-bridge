const PLUGIN_BYTES: &'static [u8] =
    include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/component.wasm");

mod host;

fn main() {
    host::run_test(PLUGIN_BYTES).expect("host_sys test should pass")
}
