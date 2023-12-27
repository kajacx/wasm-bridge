const GUEST_BYTES: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-wasi/release/wasi_components_guest.wasm");
    // include_bytes!("../../guest/target/wasm32-wasi/release/plugin_wasi.wasm");

mod host;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    host::run_test(GUEST_BYTES)
        .await
        .expect("host_sys test should pass")
}
