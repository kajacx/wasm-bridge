use wasm_bindgen::JsValue;

pub struct Instance {
    _exports: JsValue,
}

impl Instance {
    pub(crate) fn new(_exports: JsValue) -> Self {
        Self { _exports }
    }
}
