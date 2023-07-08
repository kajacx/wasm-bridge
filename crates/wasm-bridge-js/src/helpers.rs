use js_sys::Function;
use wasm_bindgen::JsValue;

pub(crate) fn warn(msg: &str) {
    let console_warn: Function = js_sys::eval("console.warn")
        .expect("eval console.warn")
        .into();

    console_warn
        .call1(&JsValue::UNDEFINED, &msg.into())
        .expect("call console.warn");
}
