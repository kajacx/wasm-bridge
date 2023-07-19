const GUEST_BYTES: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-unknown-unknown/debug/component.wasm");

const UNIVERSAL: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-unknown-unknown/debug/universal.zip");

mod host;

fn main() {
    host::run_test(GUEST_BYTES, UNIVERSAL).expect("host_sys test should pass")
}
