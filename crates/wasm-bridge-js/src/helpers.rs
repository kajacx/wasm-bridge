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

#[allow(unused)]
pub(crate) fn log_js_value(name: &str, value: &JsValue) {
    let console_log: Function = js_sys::eval("console.log")
        .expect("eval console.log")
        .into();

    console_log
        .call2(&JsValue::UNDEFINED, &name.into(), value)
        .expect("call console.log");
}
