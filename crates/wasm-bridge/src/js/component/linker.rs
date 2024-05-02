use std::{collections::HashMap, rc::Rc};

use anyhow::Context;
use js_sys::{Array, Function, Object, Reflect};
use wasm_bindgen::JsValue;

use crate::{
    direct::LazyModuleMemory, helpers::static_str_to_js, AsContextMut, DataHandle, DropHandle,
    DropHandles, Engine, Result, StoreContextMut,
};

use super::*;

static WASI_IMPORT_NAMES: &[&str] = &[
    "clock_time_get",
    "random_get",
    "fd_write",
    "fd_read",
    "environ_get",
    "environ_sizes_get",
    "proc_exit",
];

pub struct Linker<T> {
    interfaces: HashMap<String, LinkerInstance<T>>,
    wasi_interfaces: HashMap<String, LinkerInstance<T>>,
    wasi_object: Option<Box<dyn Fn() -> Object>>,
}

impl<T> Linker<T> {
    pub fn new(_engine: &Engine) -> Self {
        Self {
            interfaces: HashMap::new(),
            wasi_interfaces: HashMap::new(),
            wasi_object: None,
        }
    }

    #[deprecated(
        since = "0.4.0",
        note = "Instantiating a component synchronously can panic on the web, please use `instantiate_safe` instead."
    )]
    pub fn instantiate(
        &self,
        store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        let (imports, drop_handles, memory, wasi_info) = self.prepare_imports(store, component)?;

        if let Some(wasi_info) = wasi_info {
            component.instantiate_wasi(&imports, drop_handles, &memory, wasi_info)
        } else {
            component.instantiate(&imports, drop_handles, &memory)
        }
    }

    pub async fn instantiate_safe(
        &self,
        store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        self.instantiate_async(store, component).await
    }

    pub async fn instantiate_async(
        &self,
        store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        let (imports, drop_handles, memory, wasi_info) = self.prepare_imports(store, component)?;

        if let Some(wasi_info) = wasi_info {
            component
                .instantiate_wasi_async(&imports, drop_handles, &memory, wasi_info)
                .await
        } else {
            component
                .instantiate_async(&imports, drop_handles, &memory)
                .await
        }
    }

    fn prepare_imports(
        &self,
        mut store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<(Object, DropHandles, LazyModuleMemory, Option<WasiInfo>)> {
        let mut closures = Vec::new();

        let (imports, wasi_info) = if component.is_wasi() {
            let wasi_imports = self.wasi_object.as_ref().context("Get wasi shim object")?();
            let wasi_memory = LazyModuleMemory::new();

            for (name, interface) in self.wasi_interfaces.iter() {
                let name_js: JsValue = name.into();

                let mut imports_obj =
                    Reflect::get(&wasi_imports, &name_js).expect("imports is an object");
                if imports_obj.is_undefined() {
                    imports_obj = Object::new().into();
                }

                interface.prepare_imports(&mut store, &mut closures, &imports_obj, &wasi_memory);
                Reflect::set(&wasi_imports, &name_js, &imports_obj).expect("imports is an object");
            }

            let preview = Object::new();
            let mut setters = HashMap::<&'static str, Array>::new();
            for name in WASI_IMPORT_NAMES {
                let (func, setter) = create_dyn_fn(name);
                Reflect::set(&preview, &(*name).into(), &func).expect("preview is an object");
                setters.insert(name, setter);
            }

            let imports = Object::new();
            Reflect::set(
                &imports,
                static_str_to_js("wasi_snapshot_preview1"),
                &preview,
            )
            .expect("imports is an object");

            (imports, Some((wasi_imports, setters, wasi_memory)))
        } else {
            (Object::new(), None)
        };

        let memory = LazyModuleMemory::new();

        for (name, interface) in self.interfaces.iter() {
            let name_js: JsValue = name.into();

            let mut imports_obj = Reflect::get(&imports, &name_js).expect("imports is an object");
            if imports_obj.is_undefined() {
                imports_obj = Object::new().into();
            }

            interface.prepare_imports(&mut store, &mut closures, &imports_obj, &memory);
            Reflect::set(&imports, &name_js, &imports_obj).expect("imports is an object");
        }

        Ok((imports, Rc::new(closures), memory, wasi_info))
    }

    pub fn root(&mut self) -> &mut LinkerInstance<T> {
        self.instance("$root").unwrap()
    }

    pub fn instance<'a>(&'a mut self, name: &str) -> Result<&'a mut LinkerInstance<T>> {
        // TODO: kind of hacky, but it will work (probably)
        if name.starts_with("wasi:") {
            return self.instance_wasi(name);
        }

        // This is called at linked time, "clone" is not that bad
        Ok(self
            .interfaces
            .entry(name.to_owned())
            .or_insert_with(LinkerInstance::new))
    }

    pub fn instance_wasi<'a>(&'a mut self, name: &str) -> Result<&'a mut LinkerInstance<T>> {
        Ok(self
            .wasi_interfaces
            .entry(name.to_owned())
            .or_insert_with(LinkerInstance::new))
    }

    pub fn set_wasi_object(&mut self, creator: impl Fn() -> Object + 'static) {
        self.wasi_object = Some(Box::new(creator));
    }
}

pub struct LinkerInstance<T> {
    fns: Vec<PreparedFn<T>>,
}

impl<T> LinkerInstance<T> {
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

    pub fn resource(
        &mut self,
        _name: &str,
        _: (),
        _destroy: impl Fn(StoreContextMut<'_, T>, u32) -> Result<()>,
    ) -> Result<()> {
        Ok(())
    }

    fn prepare_imports(
        &self,
        mut store: impl AsContextMut<Data = T>,
        drop_handles: &mut Vec<DropHandle>,
        imports: &JsValue,
        memory: &LazyModuleMemory,
    ) {
        let data_handle = store.as_context_mut().data_handle();

        for function in self.fns.iter() {
            let drop_handle = function.add_to_imports(imports, data_handle.clone(), memory.clone());
            drop_handles.push(drop_handle);
        }
    }
}

struct PreparedFn<T> {
    name: String,
    creator: MakeClosure<T>,
}

#[allow(dead_code)]
impl<T> PreparedFn<T> {
    fn new(name: &str, creator: MakeClosure<T>) -> Self {
        Self {
            name: name.into(),
            creator,
        }
    }

    #[must_use]
    fn add_to_imports(
        &self,
        imports: &JsValue,
        handle: DataHandle<T>,
        memory: LazyModuleMemory,
    ) -> DropHandle {
        let (js_val, handler) = (self.creator)(handle, memory);

        Reflect::set(imports, &self.name.as_str().into(), &js_val).expect("imports is object");

        handler
    }
}

pub(crate) type DynFns = HashMap<&'static str, Array>;

fn create_dyn_fn(name: &str) -> (Function, Array) {
    let result =
        js_sys::eval(&format!("(() => {{ let arr = [() => {{ throw Error(`Not bound: {name}`); }}]; return [(...args) => arr[0](...args), arr]; }})()"))
            .expect("eval create dyn fn");

    (
        Reflect::get_u32(&result, 0)
            .expect("result is array")
            .into(),
        Reflect::get_u32(&result, 1)
            .expect("result is array")
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use js_sys::eval;

    use super::*;

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_create_dyn_fn() {
        let (func, arr) = create_dyn_fn("foo");
        assert!(func.is_function(), "dyn function is actually a function");
        assert!(arr.is_array(), "dyn array us actually an array");

        Reflect::set_u32(&arr, 0, &eval("(a, b) => a + b").unwrap()).unwrap();
        let result = func
            .call2(&JsValue::UNDEFINED, &5.into(), &3.into())
            .unwrap();
        assert_eq!(result.as_f64().unwrap(), 8.0);

        Reflect::set_u32(&arr, 0, &eval("(a, b) => a - b").unwrap()).unwrap();
        let result = func
            .call2(&JsValue::UNDEFINED, &5.into(), &3.into())
            .unwrap();
        assert_eq!(result.as_f64().unwrap(), 2.0);
    }
}

pub(crate) type WasiInfo = (Object, DynFns, LazyModuleMemory);
