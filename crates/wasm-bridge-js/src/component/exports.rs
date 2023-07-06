use wasm_bindgen::JsValue;

use crate::Result;

use super::TypedFunc;

pub struct Exports {
    root: ExportsRoot,
}

impl Exports {
    pub(crate) fn new(exports: JsValue) -> Self {
        Self {
            root: ExportsRoot::new(exports),
        }
    }

    pub fn root(&self) -> &ExportsRoot {
        &self.root
    }
}

pub struct ExportsRoot {
    exports: JsValue,
}

impl ExportsRoot {
    pub(crate) fn new(exports: JsValue) -> Self {
        Self { exports }
    }

    pub fn typed_func<Params, Return>(&self, _name: &str) -> Result<TypedFunc<Params, Return>> {
        Ok(TypedFunc::new(super::Func {}))
    }
}
