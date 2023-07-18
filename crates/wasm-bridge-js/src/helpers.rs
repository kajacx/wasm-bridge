use std::fmt::Debug;

use js_sys::Function;
use wasm_bindgen::JsValue;

#[allow(unused)]
pub(crate) fn warn(msg: &str) {
    let console_warn: Function = js_sys::eval("console.warn").unwrap().into();

    console_warn
        .call1(&JsValue::UNDEFINED, &msg.into())
        .expect("call console.warn");
}

#[allow(unused)]
pub(crate) fn log_js_value(name: &str, value: &JsValue) {
    let console_log: Function = js_sys::eval("console.log").unwrap().into();

    console_log
        .call2(&JsValue::UNDEFINED, &name.into(), value)
        .expect("call console.log");
}

#[allow(unused)]
pub(crate) fn console_log(value: impl Debug) {
    let console_log: Function = js_sys::eval("console.log").unwrap().into();

    console_log
        .call1(&JsValue::UNDEFINED, &format!("{value:?}").into())
        .expect("call console.log");
}
