use std::rc::Rc;

use crate::{helpers::map_js_error, *};
use anyhow::bail;
use js_sys::{Function, Object, Reflect, WebAssembly};
use wasm_bindgen::JsValue;

pub struct Instance {
    #[allow(unused)] // TODO: maybe this will be needed for memory access, otherwise remove
    instance: WebAssembly::Instance,
    exports: JsValue,
    _closures: Rc<Vec<DropHandler>>, // TODO: use Rc<Vec<..>> or Rc<[..]> ?
}

impl Instance {
    pub fn new(_store: impl AsContextMut, module: &Module, _: impl AsRef<[()]>) -> Result<Self> {
        let imports = Object::new();
        Self::new_with_imports(module, &imports, vec![])
    }

    pub(crate) fn new_with_imports(
        module: &Module,
        imports: &Object,
        closures: Vec<DropHandler>,
    ) -> Result<Self> {
        let instance = WebAssembly::Instance::new(&module.module, imports)
            .map_err(map_js_error("Instantiate WebAssembly module"))?;
        let exports = Reflect::get(instance.as_ref(), &"exports".into())
            .map_err(map_js_error("Get instance's exports"))?;
        Ok(Self {
            instance,
            exports,
            _closures: Rc::new(closures),
        })
    }

    pub fn get_func(&self, _store: impl AsContextMut, name: &str) -> Option<Func> {
        let function = self.get_func_inner(name).ok()?;

        Some(Func::new(function, self._closures.clone()))
    }

    pub fn get_typed_func<Params: ToJsValue, Results: FromJsValue>(
        &self,
        _store: impl AsContextMut,
        name: &str,
    ) -> Result<TypedFunc<Params, Results>, Error> {
        let function = self.get_func_inner(name)?;

        if function.length() != Params::number_of_args() {
            bail!(
                "Exported function {name} should have {} arguments, but it has {} instead.",
                Params::number_of_args(),
                function.length(),
            );
        }

        Ok(TypedFunc::new(function, self._closures.clone()))
    }

    fn get_func_inner(&self, name: &str) -> Result<Function> {
        let function = Reflect::get(&self.exports, &name.into())
            .map_err(map_js_error("Get exported fn with reflect"))?;

        if !function.is_function() {
            bail!("Exported function with name '{name}' not found");
        }

        Ok(function.into())
    }
}
