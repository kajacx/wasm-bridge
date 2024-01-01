use std::{collections::HashMap, future::Future, rc::Rc};

use heck::ToLowerCamelCase;
use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

use crate::{
    direct::{Lift, Lower, ModuleMemory},
    AsContextMut, DataHandle, DropHandle, DropHandles, Engine, Result, StoreContextMut,
};

use super::*;

pub struct Linker<T> {
    interfaces: HashMap<String, LinkerInterface<T>>,
    wasi_object: Option<Box<dyn Fn() -> Object>>,
}

impl<T> Linker<T> {
    pub fn new(_engine: &Engine) -> Self {
        Self {
            interfaces: HashMap::new(),
            wasi_object: None,
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
        let imports = self.wasi_object.as_ref().map_or_else(Object::new, |f| f());

        let mut closures = Vec::new();
        let memory = ModuleMemory::new();

        for (name, interface) in self.interfaces.iter() {
            let name_js: JsValue = name.into();

            //   if name != "$root" && name != "component-test:wit-protocol/host-add" {
            //       panic!("MODULE NAME: {name}");
            //   }

            let mut imports_obj = Reflect::get(&imports, &name_js).expect("imports is an object");
            if imports_obj.is_undefined() {
                imports_obj = Object::new().into();
            }

            interface.prepare_imports(&mut store, &mut closures, &imports_obj, memory.clone());
            Reflect::set(&imports, &name_js, &imports_obj).expect("imports is an object");
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

    pub fn set_wasi_object(&mut self, creator: impl Fn() -> Object + 'static) {
        self.wasi_object = Some(Box::new(creator));
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

    pub fn func_wrap_async<Params, Results, F>(&mut self, name: &str, _func: F) -> Result<()>
    where
        T: 'static,
        // F: IntoMakeClosure<T, Params, Results>,
        F: for<'a> Fn(
                StoreContextMut<'a, T>,
                Params,
            ) -> Box<dyn Future<Output = Result<Results>> + Send + 'a>
            + Send
            + Sync
            + 'static,
        Params: Lift + 'static,
        Results: Lower + 'static,
    {
        // self.func_wrap(name, func)
        todo!("implement func_wrap_async for {name}")
    }

    pub fn resource<U>(
        &mut self,
        _name: &str,
        _destroy: impl Fn(StoreContextMut<'_, T>, u32) -> Result<()>,
    ) -> Result<()> {
        Ok(())
    }

    fn prepare_imports(
        &self,
        mut store: impl AsContextMut<Data = T>,
        drop_handles: &mut Vec<DropHandle>,
        imports: &JsValue,
        memory: ModuleMemory,
    ) {
        let data_handle = store.as_context_mut().data_handle();

        for function in self.fns.iter() {
            let drop_handle =
                function.add_to_imports(&imports, data_handle.clone(), memory.clone());
            drop_handles.push(drop_handle);
        }
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
