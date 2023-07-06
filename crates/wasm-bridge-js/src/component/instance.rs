use std::marker::PhantomData;

use wasm_bindgen::JsValue;

use super::*;
use crate::{AsContextMut, Result};

pub struct Instance {
    exports: Exports,
}

impl Instance {
    pub(crate) fn new(exports: JsValue) -> Self {
        Self {
            exports: Exports::new(exports),
        }
    }

    pub fn exports(&self, _store: impl AsContextMut) -> &Exports {
        &self.exports
    }
}

pub struct InstancePre<T> {
    _phantom: PhantomData<T>,
}

impl<T> InstancePre<T> {
    pub fn instantiate(&self, _store: impl AsContextMut<Data = T>) -> Result<Instance> {
        todo!()
    }
}
