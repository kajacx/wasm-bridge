
use wasm_bindgen::JsValue;

pub struct Instance {
    exports: JsValue,
}

impl Instance {
    pub(crate) fn new(exports: JsValue) -> Self {
        Self { exports }
    }
}

