use std::marker::PhantomData;

use js_sys::WebAssembly;
use wasm_bindgen::JsValue;

use super::*;
use crate::{AsContextMut, Result};

pub struct Instance {
    exports: Exports,
}

impl Instance {
    // pub(crate) fn new(exports: ExportsRoot, closures: Rc<[DropHandle]>) -> Self {
    //     Self {
    //         exports: Exports::new(exports),
    //         _closures: closures,
    //     }
    // }

    pub fn new(instance: WebAssembly::Instance) -> Result<Self> {
        let js_exports: JsValue = instance.exports().into();
        let exports_root = ExportsRoot::new(js_exports)?;
        let exports = Exports::new(exports_root);

        Ok(Self { exports })
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

    pub fn instantiate_async(&self, _store: impl AsContextMut<Data = T>) -> Result<Instance> {
        todo!()
    }
}
