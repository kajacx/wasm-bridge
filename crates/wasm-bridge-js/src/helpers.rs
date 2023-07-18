use std::fmt::Debug;

use js_sys::Function;
use wasm_bindgen::JsValue;

use crate::Error;

#[allow(unused)]
pub(crate) fn warn(msg: &str) {
    let console_warn: Function = js_sys::eval("console.warn").unwrap().into();

    console_warn
        .call1(&JsValue::UNDEFINED, &msg.into())
        .unwrap();
}

#[allow(unused)]
pub(crate) fn log_js_value(name: &str, value: &JsValue) {
    let console_log: Function = js_sys::eval("console.log").unwrap().into();

    console_log
        .call2(&JsValue::UNDEFINED, &name.into(), value)
        .unwrap();
}

#[allow(unused)]
pub(crate) fn log_js_value_error(name: &str, value: &JsValue) {
    let console_error: Function = js_sys::eval("console.error").unwrap().into();

    console_error
        .call2(&JsValue::UNDEFINED, &name.into(), value)
        .unwrap();
}

#[allow(unused)]
pub(crate) fn console_log(value: impl Debug) {
    let console_log: Function = js_sys::eval("console.log").unwrap().into();

    console_log
        .call1(&JsValue::UNDEFINED, &format!("{value:?}").into())
        .unwrap();
}

pub(crate) fn map_js_error<T: Debug + AsRef<JsValue>>(hint: &'static str) -> impl Fn(T) -> Error {
    move |value: T| {
        log_js_value(hint, value.as_ref());
        anyhow::anyhow!(
            "{}, error value: {:?}, see console.log for detail.",
            hint,
            value
        )
    }
}
