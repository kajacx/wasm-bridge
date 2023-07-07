use std::{cell::RefCell, rc::Rc};

use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

use crate::*;

pub struct Linker<T> {
    fns: RefCell<Vec<PreparedFn<T>>>,
}

impl<T> Linker<T> {
    pub fn new(_engine: &Engine) -> Self {
        Self {
            fns: RefCell::new(vec![]),
        }
    }

    pub fn instantiate(
        &self,
        store: impl AsContextMut<Data = T>,
        module: &Module,
    ) -> Result<Instance, Error> {
        let store = store.as_context();

        let imports: JsValue = Object::new().into();
        let mut drop_handles = vec![];

        // FIXME: this make the linker not re-usable
        // A refactor of the IntoClosure trait is needed
        // This is a BREAKING CHANGE
        for func in self.fns.borrow_mut().drain(..) {
            let drop_handle = func.add_to_imports(&imports, store.data_handle());
            drop_handles.push(drop_handle);
        }

        Instance::new_with_imports(module, &imports.into(), drop_handles)
    }

    pub fn func_wrap<Params, Results, F>(
        &mut self,
        module: &str,
        name: &str,
        func: F,
    ) -> Result<&mut Self, Error>
    where
        F: IntoClosure<T, Params, Results> + 'static,
    {
        let creator = move |handle: DataHandle<T>| func.into_closure(handle);

        self.fns
            .borrow_mut()
            .push(PreparedFn::new(module, name, creator));

        Ok(self)
    }
}

#[derive(Clone, Debug)]
pub struct DropHandler(Rc<dyn std::fmt::Debug>);

impl DropHandler {
    pub fn new<T: std::fmt::Debug + 'static>(value: T) -> Self {
        Self(Rc::new(value))
    }
}

struct PreparedFn<T> {
    module: String,
    name: String,
    creator: Box<dyn FnOnce(DataHandle<T>) -> (JsValue, DropHandler)>,
}

impl<T> PreparedFn<T> {
    fn new(
        module: &str,
        name: &str,
        creator: impl FnOnce(DataHandle<T>) -> (JsValue, DropHandler) + 'static,
    ) -> Self {
        Self {
            module: module.into(),
            name: name.into(),
            creator: Box::new(creator),
        }
    }

    #[must_use]
    fn add_to_imports(self, imports: &JsValue, handle: &DataHandle<T>) -> DropHandler {
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
