use std::collections::HashMap;

use heck::ToLowerCamelCase;
use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

use crate::{
    direct_bytes::ModuleMemory, helpers::static_str_to_js, AsContextMut, DataHandle, DropHandle,
    Engine, Result,
};

use super::*;

pub struct Linker<T> {
    fns: Vec<PreparedFn<T>>,
    instances: HashMap<String, Linker<T>>,
    #[allow(unused)] // TODO: re-enable wasi
    wasi_imports: Option<Object>,
}

impl<T> Linker<T> {
    pub fn new(_engine: &Engine) -> Self {
        Self {
            fns: vec![],
            instances: HashMap::new(),
            wasi_imports: None,
        }
    }

    pub fn instantiate(
        &self,
        _store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        // let import_object = js_sys::Object::new();
        // if let Some(imports) = &self.wasi_imports {
        //     js_sys::Object::assign(&import_object, imports);
        // }
        // let import_object: JsValue = import_object.into();

        // let mut closures = Vec::with_capacity(self.fns.len());
        // let data_handle = store.as_context_mut().data_handle();

        // for function in self.fns.iter() {
        //     let drop_handle = function.add_to_imports(&import_object, data_handle.clone());
        //     closures.push(drop_handle);
        // }

        // for (instance_name, instance_linker) in self.instances.iter() {
        //     let instance_obj = Object::new();

        //     for function in instance_linker.fns.iter() {
        //         let drop_handle =
        //             function.add_to_instance_imports(&instance_obj, data_handle.clone());
        //         closures.push(drop_handle);
        //     }

        //     Reflect::set(&import_object, &instance_name.into(), &instance_obj).unwrap();
        // }

        // let closures = Rc::from(closures);
        // component.instantiate(store, &import_object, closures)
        component.instantiate()
    }

    // TODO: async was removed thanks to the macro
    pub async fn instantiate_async(
        &self,
        _store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        component.instantiate_async().await
    }

    pub fn root(&mut self) -> &mut Self {
        self
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

    pub fn func_wrap_async<Params, Results, F>(&mut self, name: &str, func: F) -> Result<()>
    where
        T: 'static,
        F: IntoMakeClosure<T, Params, Results>,
    {
        self.func_wrap(name, func)
    }

    pub fn instance<'a>(&'a mut self, name: &str) -> Result<&'a mut Linker<T>> {
        // This is called at linked time, "clone" is not that bad
        Ok(self
            .instances
            .entry(name.to_owned())
            .or_insert_with(|| Linker::new(&Engine {}))) // TODO: engine re-creation
    }

    #[cfg(feature = "wasi")]
    pub(crate) fn set_wasi_imports(&mut self, imports: Object) {
        self.wasi_imports = Some(imports);
    }
}

#[allow(unused)] // TODO: this is for imports
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

        let object: JsValue = Object::new().into();
        Reflect::set(&object, static_str_to_js("default"), &js_val).expect("object is object");

        Reflect::set(imports, &self.name.as_str().into(), &object).expect("imports is object");

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
