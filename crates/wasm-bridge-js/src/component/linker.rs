use wasm_bindgen::JsValue;

use crate::{component::*, FromJsResults, IntoJsParams, StoreContextMut};
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
        store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        let import_object: JsValue = js_sys::Object::new().into();

        component.instantiate(store, &import_object)
    }

    pub fn root(&mut self) -> &mut Self {
        self
    }

    pub fn func_wrap<Params, Results, F>(&mut self, name: &str, func: F) -> Result<()>
    where
        Params: IntoJsParams,
        Results: FromJsResults,
        F: Fn(StoreContextMut<T>, Params) -> Result<Results>,
    {
        Ok(())
    }
}
