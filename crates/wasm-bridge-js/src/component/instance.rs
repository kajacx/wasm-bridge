use std::marker::PhantomData;

use wasm_bindgen::JsValue;

use crate::{AsContextMut, Result};

use super::TypedFunc;

pub struct Instance {
    _exports: JsValue,
}

impl Instance {
    pub(crate) fn _new(_exports: JsValue) -> Self {
        Self { _exports }
    }

    pub fn exports(&self, _store: impl AsContextMut) -> Exports {
        Exports {
            root: ExportsRoot {},
        }
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

pub struct Exports {
    root: ExportsRoot,
}

impl Exports {
    pub fn root(&self) -> &ExportsRoot {
        &self.root
    }
}

pub struct ExportsRoot {}
impl ExportsRoot {
    pub fn typed_func<Params, Return>(&self, _name: &str) -> Result<TypedFunc<Params, Return>> {
        Ok(TypedFunc::new(super::Func {}))
    }
}
