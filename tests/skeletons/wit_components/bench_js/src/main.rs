const GUEST_BYTES: &'static [u8] =
    include_bytes!("../../guest/target/wasm32-unknown-unknown/debug/wit_components_guest.wasm");

mod host;

#[wasm_bindgen_test::wasm_bindgen_test]
fn main() {
    host::run_test(GUEST_BYTES).expect("host_js test should pass")
}

fn bench<T>(name: &str, code: impl FnMut() -> T) {
    let bench = easybench_wasm::bench(code);
    log(&format!(
        "OLD CODE: {name}: {} ns per iteration",
        bench.ns_per_iter
    ));
    log(&format!("{:?}", bench));
}

fn log(msg: &str) {
    use wasm_bridge::js_sys;
    use wasm_bridge::wasm_bindgen;

    let console_log: js_sys::Function =
        js_sys::eval("console.log").expect("get console.log").into();

    console_log
        .call1(&wasm_bindgen::JsValue::UNDEFINED, &msg.into())
        .expect("call console.log with message");
}
