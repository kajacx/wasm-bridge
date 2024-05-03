const GUEST_BYTES: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-unknown-unknown/debug/wit_components_guest.wasm");

mod host;

#[test]
fn main() {
    host::run_test(GUEST_BYTES).expect("host_sys test should pass")
}

fn bench<T>(name: &str, code: impl FnMut() -> T) {
    let bench = easybench::bench(code);
    println!("SYS BENCH: {name}: {bench:?}");
}
