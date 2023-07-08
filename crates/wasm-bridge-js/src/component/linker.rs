use js_sys::Reflect;
use wasm_bindgen::JsValue;

use crate::{
    AsContextMut, DropHandler, Engine, FromJsResults, IntoJsParams, Result, StoreContext,
    StoreContextMut,
};

use super::*;

pub struct Linker<T> {
    fns: Vec<PreparedFn<T>>,
}

impl<T> Linker<T> {
    pub fn new(_engine: &Engine) -> Self {
        Self { fns: vec![] }
    }

    pub fn instantiate(
        &self,
        mut store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        let import_object: JsValue = js_sys::Object::new().into();

        for function in self.fns.iter() {
            function.add_to_imports(&import_object, store.as_context_mut());
        }

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

struct PreparedFn<T> {
    name: String,
    creator: MakeClosure<T>,
}

impl<T> PreparedFn<T> {
    fn new(name: &str, creator: MakeClosure<T>) -> Self {
        Self {
            name: name.into(),
            creator,
        }
    }

    #[must_use]
    fn add_to_imports(&self, imports: &JsValue, handle: StoreContextMut<T>) -> DropHandler {
        let (js_val, handler) = (self.creator)(handle);

        // FIXME: change import name!
        Reflect::set(&imports, &self.name.as_str().into(), &js_val).expect("imports is object");

        handler
    }
}
