#[cfg(test)]
const GUEST_BYTES: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-unknown-unknown/release/wit_components_guest.wasm");

#[cfg(test)]
mod host;

#[test]
fn main() {
    host::run_test(GUEST_BYTES).expect("host_sys test should pass")
}
