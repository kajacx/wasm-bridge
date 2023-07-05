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
        _component: &Component,
    ) -> Result<Instance> {
        // let compile_core = js_sys::eval(compile_core)?;

        // let import_object: JsValue = js_sys::Object::new().into();

        // let instance =
        //     component
        //         .instantiate
        //         .call2(&component.component, &compile_core, &import_object)?;

        // Ok(Instance::new(instance))
        todo!()
    }
}
