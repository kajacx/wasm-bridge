use std::fmt::Debug;

use js_sys::{Function, JsString};
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
pub(crate) fn console_log(value: impl Debug) {
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

/// From: https://github.com/cloudflare/serde-wasm-bindgen/blob/main/src/lib.rs
#[inline]
pub fn static_str_to_js(s: &'static str) -> JsString {
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[derive(Default)]
    struct PtrHasher {
        addr: usize,
    }

    impl std::hash::Hasher for PtrHasher {
        fn write(&mut self, _bytes: &[u8]) {
            unreachable!();
        }

        fn write_usize(&mut self, addr_or_len: usize) {
            if self.addr == 0 {
                self.addr = addr_or_len;
            }
        }

        fn finish(&self) -> u64 {
            self.addr as _
        }
    }

    type PtrBuildHasher = std::hash::BuildHasherDefault<PtrHasher>;

    thread_local! {
        // Since we're mainly optimising for converting the exact same string literal over and over again,
        // which will always have the same pointer, we can speed things up by indexing by the string's pointer
        // instead of its value.
        static CACHE: RefCell<HashMap<*const str, JsString, PtrBuildHasher>> = Default::default();
    }

    CACHE.with(|cache| {
        cache
            .borrow_mut()
            .entry(s)
            .or_insert_with(|| {
                tracing::debug!(?s, "adding static str to cache");
                s.into()
            })
            .clone()
    })
}
