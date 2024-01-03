const GUEST_BYTES: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-unknown-unknown/release/no_bindgen_guest.wasm");

mod host;

#[tokio::test(flavor = "current_thread")]
async fn test() {
    host::run_test(GUEST_BYTES).await.expect("host_sys test should pass")
}
