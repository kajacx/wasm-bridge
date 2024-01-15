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

pub fn log_js_value(name: &str, value: &JsValue) {
    let console_log: Function = js_sys::eval("console.log").unwrap().into();

    console_log
        .call3(&JsValue::UNDEFINED, &name.into(), &value.js_typeof(), value)
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
pub fn console_log(value: impl Debug) {
    let console_log: Function = js_sys::eval("console.log").unwrap().into();

    console_log
        .call1(&JsValue::UNDEFINED, &format!("{value:?}").into())
        .unwrap();
}

pub fn map_js_error<T: Debug + AsRef<JsValue>>(hint: &'static str) -> impl Fn(T) -> Error {
    move |value: T| {
        if cfg!(feature = "error-logging") {
            log_js_value(hint, value.as_ref());
            anyhow::anyhow!(
                "{}, error value: {:?}, see console.error log for detail.",
                hint,
                value
            )
        } else {
            anyhow::anyhow!(
                "{}, error value: {:?}, enable 'error-logging' feature to log value to console.error.",
                hint,
                value
            )
        }
    }
}

pub fn static_str_to_js(s: &'static str) -> &'static JsValue {
    use std::cell::RefCell;
    use std::collections::HashMap;

    thread_local! {
        // Since we're mainly optimizing for converting the exact same string literal over and over again,
        // which will always have the same pointer, we can speed things up by indexing by the string's pointer
        // instead of its value.
        static CACHE: RefCell<HashMap<(*const u8, usize), &'static JsValue>> = Default::default();
    }

    let key = (s.as_ptr(), s.len());

    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let reference = cache.entry(key).or_insert_with(|| {
            let js_val: JsValue = s.into();
            let boxed = Box::new(js_val);
            let leaked = Box::leak(boxed);
            leaked as &'static JsValue
        });
        *reference
        // .clone()
        // .clone()
        //let val = cache.borrow_mut().entry(key).or_insert_with(|| s.into());
        // unsafe { &*(val as *const JsString) }
        //val.clone()
        //val as &'static JsString
    })
}
