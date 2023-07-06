use wasm_bindgen::JsValue;

use crate::component::*;
use crate::{AsContextMut, Engine, Result};
use std::marker::PhantomData;

pub struct Linker<T> {
    _phantom: PhantomData<T>,
}

impl<T> Linker<T> {
    pub fn new(_engine: &Engine) -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub fn instantiate(
        &self,
        _store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        let import_object: JsValue = js_sys::Object::new().into();

        component.instantiate(&import_object)
    }
}
