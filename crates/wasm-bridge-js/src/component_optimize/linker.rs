use std::{collections::HashMap, future::Future, rc::Rc};

use js_sys::{Array, Function, Object, Reflect};
use wasm_bindgen::JsValue;

use crate::{
    direct::{Lift, Lower, ModuleMemory},
    helpers::static_str_to_js,
    AsContextMut, DataHandle, DropHandle, DropHandles, Engine, Result, StoreContextMut,
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
    interfaces: HashMap<String, LinkerInterface<T>>,
    wasi_interfaces: HashMap<String, LinkerInterface<T>>,
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

    pub fn instantiate(
        &self,
        store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        let (imports, drop_handles, inflating_fns, wasi_info) =
            self.prepare_imports(store, component)?;

        if let Some(wasi_info) = wasi_info {
            component.instantiate_wasi(&imports, drop_handles, &inflating_fns, wasi_info)
        } else {
            component.instantiate(&imports, drop_handles, &inflating_fns)
        }
    }

    pub async fn instantiate_async(
        &self,
        store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<Instance> {
        let (imports, drop_handles, inflating_fns, wasi_info) =
            self.prepare_imports(store, component)?;

        if let Some(wasi_info) = wasi_info {
            component
                .instantiate_wasi_async(&imports, drop_handles, &inflating_fns, wasi_info)
                .await
        } else {
            component
                .instantiate_async(&imports, drop_handles, &inflating_fns)
                .await
        }
    }

    fn prepare_imports(
        &self,
        mut store: impl AsContextMut<Data = T>,
        component: &Component,
    ) -> Result<(
        Object,
        DropHandles,
        HashMap<String, InflatingDynFns>,
        Option<WasiInfo>,
    )> {
        let mut closures = Vec::new();

        let (imports, wasi_info) = if let (Some(wasi_object), Some(_wasi_core)) =
            (&self.wasi_object, &component.module_core2)
        {
            let wasi_imports = wasi_object();
            let mut inflating_wasi_imports = HashMap::<String, InflatingDynFns>::new();

            for (name, interface) in self.wasi_interfaces.iter() {
                let name_js: JsValue = name.into();
                let mut wasi_dyn_fns = InflatingDynFns::new();

                let mut imports_obj =
                    Reflect::get(&wasi_imports, &name_js).expect("imports is an object");
                if imports_obj.is_undefined() {
                    imports_obj = Object::new().into();
                }

                interface.prepare_imports(&imports_obj, &mut wasi_dyn_fns);
                inflating_wasi_imports.insert(name.to_owned(), wasi_dyn_fns);

                Reflect::set(&wasi_imports, &name_js, &imports_obj).expect("imports is an object");
            }

            let preview = Object::new();
            let mut setters = HashMap::<&'static str, Array>::new();
            for name in WASI_IMPORT_NAMES {
                let (func, setter) = create_dyn_fn(name);
                Reflect::set(&preview, &(*name).into(), &func).expect("preview is object");
                setters.insert(name, setter);
            }

            let imports = Object::new();
            Reflect::set(
                &imports,
                static_str_to_js("wasi_snapshot_preview1"),
                &preview,
            )
            .expect("imports is object");

            (
                imports,
                Some((wasi_imports, setters, inflating_wasi_imports)),
            )
        } else {
            (Object::new(), None)
        };

        let mut inflating_imports = HashMap::<String, InflatingDynFns>::new();

        for (name, interface) in self.interfaces.iter() {
            let name_js: JsValue = name.into();

            let mut imports_obj = Reflect::get(&imports, &name_js).expect("imports is an object");
            if imports_obj.is_undefined() {
                imports_obj = Object::new().into();
            }

            let mut dyn_fns = InflatingDynFns::new();
            interface.prepare_imports(&imports_obj, &mut dyn_fns);
            inflating_imports.insert(name.to_owned(), dyn_fns);

            Reflect::set(&imports, &name_js, &imports_obj).expect("imports is an object");
        }

        Ok((imports, Rc::new(closures), inflating_imports, wasi_info))
    }

    pub fn root(&mut self) -> &mut LinkerInterface<T> {
        self.instance("$root").unwrap()
    }

    pub fn instance<'a>(&'a mut self, name: &str) -> Result<&'a mut LinkerInterface<T>> {
        // TODO: kind of hacky, but it will work (probably)
        if name.starts_with("wasi:") {
            return self.instance_wasi(name);
        }

        // This is called at linked time, "clone" is not that bad
        Ok(self
            .interfaces
            .entry(name.to_owned())
            .or_insert_with(LinkerInterface::new))
    }

    pub fn instance_wasi<'a>(&'a mut self, name: &str) -> Result<&'a mut LinkerInterface<T>> {
        Ok(self
            .wasi_interfaces
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

    pub fn func_wrap_async<Params, Results, F>(&mut self, _name: &str, _func: F) -> Result<()>
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
        todo!("implement func_wrap_async for {_name}")
        // FIXME: actually implement this, or somehow resolve "normal" user imports
        // Ok(())
    }

    pub fn resource<U>(
        &mut self,
        _name: &str,
        _destroy: impl Fn(StoreContextMut<'_, T>, u32) -> Result<()>,
    ) -> Result<()> {
        Ok(())
    }

    fn prepare_imports(&self, imports: &JsValue, dyn_fns: &mut InflatingDynFns) {
        for function in self.fns.iter() {
            let drop_handle = function.prepare_import(imports, dyn_fns);
        }
    }

    fn finalize_imports(
        &self,
        mut store: impl AsContextMut<Data = T>, // TODO: this argument looks suspicious
        dyn_fns: &InflatingDynFns,
        drop_handles: &mut Vec<DropHandle>,
        memory: &ModuleMemory,
    ) {
        let data_handle = store.as_context_mut().data_handle();

        for function in self.fns.iter() {
            let drop_handle =
                function.finalize_import(dyn_fns, data_handle.clone(), memory.clone());
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
            name: name.into(),
            creator,
        }
    }

    fn prepare_import(&self, imports: &JsValue, dyn_fns: &mut InflatingDynFns) {
        let (imported_fn, array) = create_inflating_dyn_fn(&self.name);

        dyn_fns.insert(self.name.clone(), array);

        Reflect::set(imports, &self.name.as_str().into(), &imported_fn)
            .expect("imports is an object");
    }

    #[must_use]
    fn finalize_import(
        &self,
        dyn_fns: &InflatingDynFns,
        handle: DataHandle<T>,
        memory: ModuleMemory,
    ) -> DropHandle {
        let (js_val, handler) = (self.creator)(handle, memory);

        Reflect::set_u32(dyn_fns.get(&self.name).expect("get dyn fn"), 0, &js_val)
            .expect("dyn fn setter is an array");

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

pub(crate) type InflatingDynFns = HashMap<String, Array>;

fn create_inflating_dyn_fn(name: &str) -> (Function, Array) {
    let result =
        js_sys::eval(&format!("(() => {{ let arr = [() => {{ throw Error(`Not bound: {name}`); }}]; return [(...args) => arr[0](args), arr]; }})()"))
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

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_create_inflating_dyn_fn() {
        let (func, arr) = create_inflating_dyn_fn("foo");
        assert!(func.is_function(), "dyn function is actually a function");
        assert!(arr.is_array(), "dyn array us actually an array");

        Reflect::set_u32(&arr, 0, &eval("(args) => args[0] + args[1]").unwrap()).unwrap();
        let result = func
            .call2(&JsValue::UNDEFINED, &5.into(), &3.into())
            .unwrap();
        assert_eq!(result.as_f64().unwrap(), 8.0);

        Reflect::set_u32(&arr, 0, &eval("(args) => args[0] - args[1]").unwrap()).unwrap();
        let result = func
            .call2(&JsValue::UNDEFINED, &5.into(), &3.into())
            .unwrap();
        assert_eq!(result.as_f64().unwrap(), 2.0);
    }
}

pub(crate) type WasiInfo = (Object, DynFns, HashMap<String, InflatingDynFns>);
