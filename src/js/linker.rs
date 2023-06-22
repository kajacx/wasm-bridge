use std::rc::Rc;

use js_sys::{Object, Reflect};
use wasm_bindgen::{prelude::Closure, JsValue};

use crate::*;

pub struct Linker {
    import_object: JsValue,
    closures: Vec<Rc<Closure<dyn Fn(i32) -> i32>>>,
}

impl Linker {
    pub fn new(_engine: &Engine) -> Self {
        Self {
            import_object: Object::new().into(),
            closures: vec![],
        }
    }

    pub fn instantiate(&self, _store: &mut Store<()>, module: &Module) -> Result<Instance, Error> {
        let imports = self.import_object.clone().into();
        Instance::new_with_imports(module, &imports, self.closures.clone())
    }

    pub fn func_wrap<F>(&mut self, module: &str, name: &str, func: F) -> Result<&mut Self, Error>
    where
        F: Fn(Caller<()>, i32) -> i32 + 'static + Send + Sync,
    {
        let module = self.module(module)?;

        let closure =
            Closure::<dyn Fn(i32) -> i32 + 'static>::new(move |arg: i32| func(Caller::new(), arg));

        let as_js_val: JsValue = closure.as_ref().into();

        Reflect::set(&module, &name.into(), &as_js_val)?;

        self.closures.push(Rc::new(closure));

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
