const GUEST_BYTES: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-unknown-unknown/debug/no_bindgen_guest.wasm");

mod host;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    host::run_test(GUEST_BYTES).await.expect("host_sys test should pass")
}
