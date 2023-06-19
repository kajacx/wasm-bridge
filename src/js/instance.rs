use crate::*;
use js_sys::{Object, Reflect, WebAssembly};
use wasm_bindgen::JsValue;

pub struct Instance {
    instance: WebAssembly::Instance,
    exports: JsValue,
}

impl Instance {
    pub fn new(
        _store: &mut Store<()>,
        module: &Module,
        _: impl AsRef<[()]>,
    ) -> Result<Self, Error> {
        let imports = Object::new();
        let instance = WebAssembly::Instance::new(&module.module, &imports)?;
        let exports = Reflect::get(&instance.as_ref(), &"exports".into())?;
        Ok(Self { instance, exports })
    }
}

impl Instance {
    pub fn get_typed_func<Params: ToJsParams, Results: FromJsValue>(
        &self,
        _store: &mut Store<()>,
        name: &str,
    ) -> Result<TypedFunc<Params, Results>, Error> {
        let function = Reflect::get(&self.exports, &name.into())?;

        if !function.is_function() {
            // TODO: better error here?
            return Err(Error::JsError(function));
        }

        Ok(TypedFunc::new(&self.instance, function.into()))
    }
}
