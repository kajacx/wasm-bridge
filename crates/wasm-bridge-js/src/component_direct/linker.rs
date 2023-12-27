use std::{collections::HashMap, rc::Rc};

use heck::ToLowerCamelCase;
use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

use crate::{
    direct_bytes::ModuleMemory, AsContextMut, DataHandle, DropHandle, DropHandles, Engine, Result,
};

use super::*;

pub struct Linker<T> {
    interfaces: HashMap<String, LinkerInterface<T>>,
    #[allow(unused)] // TODO: re-enable wasi
    wasi_imports: Option<Object>,
}

impl<T> Linker<T> {
    pub fn new(_engine: &Engine) -> Self {
        Self {
            interfaces: HashMap::new(),
            wasi_imports: None,
        }
    }

    pub fn instantiate(
        &self,
        store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        let (imports, drop_handles, memory) = self.prepare_imports(store);
        component.instantiate(&imports, drop_handles, memory)
    }

    pub async fn instantiate_async(
        &self,
        store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        let (imports, drop_handles, memory) = self.prepare_imports(store);
        component
            .instantiate_async(&imports, drop_handles, memory)
            .await
    }

    fn prepare_imports(
        &self,
        mut store: impl AsContextMut<Data = T>,
    ) -> (Object, DropHandles, ModuleMemory) {
        let imports = js_sys::Object::new();

        let mut closures = Vec::new();
        let memory = ModuleMemory::new();

        for (name, interface) in self.interfaces.iter() {
            let interface_imports =
                interface.prepare_imports(&mut store, &mut closures, memory.clone());
            Reflect::set(&imports, &name.into(), &interface_imports).expect("imports is an object");
        }

        (imports, Rc::new(closures), memory)
    }

    pub fn root(&mut self) -> &mut LinkerInterface<T> {
        self.instance("$root").unwrap()
    }

    pub fn instance<'a>(&'a mut self, name: &str) -> Result<&'a mut LinkerInterface<T>> {
        // This is called at linked time, "clone" is not that bad
        Ok(self
            .interfaces
            .entry(name.to_owned())
            .or_insert_with(LinkerInterface::new))
    }

    #[cfg(feature = "wasi")]
    pub(crate) fn set_wasi_imports(&mut self, imports: Object) {
        self.wasi_imports = Some(imports);
    }
}

pub struct LinkerInterface<T> {
    fns: Vec<PreparedFn<T>>,
}

impl<T> LinkerInterface<T> {
    fn new() -> Self {
        Self { fns: vec![] }
    }

    pub fn func_wrap<Params, Results, F>(&mut self, name: &str, func: F) -> Result<()>
    where
        T: 'static,
        F: IntoMakeClosure<T, Params, Results>,
    {
        self.fns
            .push(PreparedFn::new(name, func.into_make_closure()));

        Ok(())
    }

    fn prepare_imports(
        &self,
        mut store: impl AsContextMut<Data = T>,
        drop_handles: &mut Vec<DropHandle>,
        memory: ModuleMemory,
    ) -> JsValue {
        let imports: JsValue = js_sys::Object::new().into();

        let data_handle = store.as_context_mut().data_handle();

        for function in self.fns.iter() {
            let drop_handle =
                function.add_to_imports(&imports, data_handle.clone(), memory.clone());
            drop_handles.push(drop_handle);
        }

        imports
    }
}

#[allow(unused)]
struct PreparedFn<T> {
    name: String,
    creator: MakeClosure<T>,
}

#[allow(dead_code)]
impl<T> PreparedFn<T> {
    fn new(name: &str, creator: MakeClosure<T>) -> Self {
        Self {
            // name: name.to_lower_camel_case(), // Import name is in kebab-case on purpose
            name: name.into(),
            creator,
        }
    }

    #[must_use]
    fn add_to_imports(
        &self,
        imports: &JsValue,
        handle: DataHandle<T>,
        memory: ModuleMemory,
    ) -> DropHandle {
        let (js_val, handler) = (self.creator)(handle, memory);

        Reflect::set(imports, &self.name.as_str().into(), &js_val).expect("imports is object");

        handler
    }

    #[must_use]
    fn add_to_instance_imports(
        &self,
        imports: &JsValue,
        handle: DataHandle<T>,
        memory: ModuleMemory,
    ) -> DropHandle {
        let (js_val, handler) = (self.creator)(handle, memory);

        Reflect::set(imports, &self.name.to_lower_camel_case().into(), &js_val)
            .expect("imports is object");

        handler
    }
}
