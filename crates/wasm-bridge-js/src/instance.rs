use crate::*;
use js_sys::{Function, Object, Reflect, WebAssembly};
use wasm_bindgen::JsValue;

pub struct Instance {
    instance: WebAssembly::Instance,
    exports: JsValue,
    _closures: Vec<DropHandler>,
}

impl Instance {
    pub fn new(
        _store: impl AsContextMut,
        module: &Module,
        _: impl AsRef<[()]>,
    ) -> Result<Self, Error> {
        let imports = Object::new();
        Self::new_with_imports(module, &imports, vec![])
    }

    pub(crate) fn new_with_imports(
        module: &Module,
        imports: &Object,
        closures: Vec<DropHandler>,
    ) -> Result<Self, Error> {
        let instance = WebAssembly::Instance::new(&module.module, imports)?;
        let exports = Reflect::get(instance.as_ref(), &"exports".into())?;
        Ok(Self {
            instance,
            exports,
            _closures: closures,
        })
    }

    pub fn get_typed_func<Params: IntoJsParams, Results: FromJsValue>(
        &self,
        _store: impl AsContextMut,
        name: &str,
    ) -> Result<TypedFunc<Params, Results>, Error> {
        let function = Reflect::get(&self.exports, &name.into())?;

        if !function.is_function() {
            return Err(Error::ExportedFnNotFound(name.into()));
        }

        let function: Function = function.into();

        if function.length() != Params::number_of_args() {
            return Err(Error::IncorrectNumOfArgs(
                name.into(),
                Params::number_of_args(),
                function.length(),
            ));
        }

        Ok(TypedFunc::new(&self.instance, function))
    }
}
