const GUEST_BYTES: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-unknown-unknown/debug/wit_components_guest.wasm");

mod host;

fn main() {
    host::run_test(GUEST_BYTES).expect("host_sys test should pass")
}
