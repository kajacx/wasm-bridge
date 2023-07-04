use std::rc::Rc;

use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

use crate::*;

pub struct Linker {
    import_object: JsValue,
    closures: Vec<DropHandler>,
}

impl Linker {
    pub fn new(_engine: &Engine) -> Self {
        Self {
            import_object: Object::new().into(),
            closures: vec![],
        }
    }

    pub fn instantiate(
        &self,
        _store: impl AsContextMut,
        module: &Module,
    ) -> Result<Instance, Error> {
        let imports = self.import_object.clone().into();
        Instance::new_with_imports(module, &imports, self.closures.clone())
    }

    pub fn func_wrap<Params, Results, F>(
        &mut self,
        module: &str,
        name: &str,
        func: F,
    ) -> Result<&mut Self, Error>
    where
        F: IntoClosure<Params, Results>,
    {
        let module = self.module(module)?;

        let (js_val, handler) = func.into_closure();

        Reflect::set(&module, &name.into(), &js_val)?;

        self.closures.push(handler);

        Ok(self)
    }

    fn module(&self, module: &str) -> Result<JsValue, Error> {
        let module_str: JsValue = module.into();
        let existing = Reflect::get(&self.import_object, &module_str)?;

        if existing.is_object() {
            Ok(existing)
        } else {
            let new_module: JsValue = Object::new().into();
            Reflect::set(&self.import_object, &module_str, &new_module)?;
            Ok(new_module)
        }
    }
}

#[derive(Clone, Debug)]
pub struct DropHandler(Rc<dyn std::fmt::Debug>);

impl DropHandler {
    pub fn new<T: std::fmt::Debug + 'static>(value: T) -> Self {
        Self(Rc::new(value))
    }
}
