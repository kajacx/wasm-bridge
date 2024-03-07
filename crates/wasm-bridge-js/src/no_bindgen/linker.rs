use std::{rc::Rc, sync::Arc};

use js_sys::{Array, Function, Object, Reflect};
use wasm_bindgen::{prelude::*, JsValue};

use crate::*;

pub struct Linker<T> {
    fns: Vec<PreparedFn<T>>,
}

impl<T> Linker<T> {
    pub fn new(_engine: &Engine) -> Self {
        Self { fns: vec![] }
    }

    pub fn instantiate(
        &self,
        store: impl AsContextMut<Data = T>,
        module: &Module,
    ) -> Result<Instance, Error> {
        let (imports, drop_handles) = self.collect_imports(store);
        Instance::new_with_imports(module, &imports, drop_handles)
    }

    pub async fn instantiate_async(
        &self,
        store: impl AsContextMut<Data = T>,
        module: &Module,
    ) -> Result<Instance> {
        let (imports, drop_handles) = self.collect_imports(store);
        Instance::new_with_imports_async(module, &imports, drop_handles).await
    }

    fn collect_imports(&self, store: impl AsContextMut<Data = T>) -> (Object, Vec<DropHandle>) {
        let store = store.as_context();

        let imports = Object::new();
        let mut drop_handles = vec![];

        for func in self.fns.iter() {
            let drop_handle = func.add_to_imports(&imports, store.data_handle());
            drop_handles.push(drop_handle);
        }

        (imports, drop_handles)
    }

    pub fn func_new<F>(
        &mut self,
        module: &str,
        name: &str,
        _type: FuncType,
        func: F,
    ) -> Result<&mut Self>
    where
        F: Fn(Caller<T>, &[Val], &mut [Val]) -> Result<()> + 'static,
        T: 'static,
    {
        let func_rc = Rc::new(func);
        let creator = move |handle: DataHandle<T>| {
            let caller = Caller::new(handle);
            let func_clone = func_rc.clone();

            let closure =
                Closure::<dyn Fn(Array) -> Result<JsValue, JsValue>>::new(move |js_args: Array| {
                    let mut args = Vec::with_capacity(js_args.length() as _);
                    for index in 0..args.capacity() {
                        let js_val = Reflect::get_u32(&js_args, index as _)?;
                        args.push(Val::from_js_value(&js_val).map_err::<JsValue, _>(|e| {
                            format!("Cannot convert JsValue to Val: {e:}").into()
                        })?);
                    }

                    // TODO: support different amounts of return values? HOW????
                    let mut rets = vec![Val::I32(0)];

                    func_clone(caller.clone(), &args, &mut rets).map_err::<JsValue, _>(|e| {
                        format!("Error in imported function: {e:?}").into()
                    })?;

                    Ok(rets[0].to_js_value())
                });

            let (js_func, handler) = DropHandle::from_closure(closure);
            let js_func = transform_dynamic_closure_arguments(js_func);

            (js_func, handler)
        };

        self.fns
            .push(PreparedFn::new(module, name, Box::new(creator)));

        Ok(self)
    }

    pub fn func_wrap<Params, Results, F>(
        &mut self,
        module: &str,
        name: &str,
        func: F,
    ) -> Result<&mut Self>
    where
        F: IntoMakeClosure<T, Params, Results> + 'static,
    {
        let creator = func.into_make_closure();

        self.fns.push(PreparedFn::new(module, name, creator));

        Ok(self)
    }
}

#[derive(Debug)]
pub struct DropHandle(Box<dyn std::fmt::Debug>);
pub type DropHandles = Arc<Vec<DropHandle>>;

impl DropHandle {
    pub(crate) fn new<T: std::fmt::Debug + 'static>(value: T) -> Self {
        Self(Box::new(value))
    }

    pub(crate) fn from_closure(
        closure: impl AsRef<JsValue> + std::fmt::Debug + 'static,
    ) -> (JsValue, Self) {
        let js_value = closure.as_ref().clone();

        (js_value, Self::new(closure))
    }
}

struct PreparedFn<T> {
    module: String,
    name: String,
    creator: MakeClosure<T>,
}

impl<T> PreparedFn<T> {
    fn new(module: &str, name: &str, creator: MakeClosure<T>) -> Self {
        Self {
            module: module.into(),
            name: name.into(),
            creator,
        }
    }

    #[must_use]
    fn add_to_imports(&self, imports: &JsValue, handle: &DataHandle<T>) -> DropHandle {
        let module = Self::module(imports, &self.module);

        let (js_val, handler) = (self.creator)(handle.clone());

        Reflect::set(&module, &self.name.as_str().into(), &js_val).expect("module is object");

        handler
    }

    fn module(imports: &JsValue, module: &str) -> JsValue {
        let module_str: JsValue = module.into();
        let existing = Reflect::get(imports, &module_str).expect("imports is object");

        if existing.is_object() {
            existing
        } else {
            let new_module: JsValue = Object::new().into();
            Reflect::set(imports, &module_str, &new_module).expect("imports is object");
            new_module
        }
    }
}

fn transform_dynamic_closure_arguments(closure: JsValue) -> JsValue {
    let transformer: Function = js_sys::eval(r#"(func) => (...args) => func(args)"#)
        .unwrap()
        .into();
    debug_assert!(transformer.is_function(), "transformer is a function");

    transformer.call1(&JsValue::UNDEFINED, &closure).unwrap()
}

pub async fn instantiate_async<T>(
    store: impl AsContextMut<Data = T>,
    linker: &Linker<T>,
    module: &Module,
) -> Result<Instance> {
    linker.instantiate_async(store, module).await
}
