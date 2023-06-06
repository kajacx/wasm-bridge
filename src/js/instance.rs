use crate::*;
use js_sys::{Object, Reflect, WebAssembly};

pub struct Instance {
    instance: WebAssembly::Instance,
    // exports: Array,
}

impl Instance {
    pub fn new(
        _store: &mut Store<()>,
        module: &Module,
        _: impl AsRef<[()]>,
    ) -> Result<Self, Error> {
        let imports = Object::new();
        let instance =
            WebAssembly::Instance::new(&module.module, &imports).expect("TODO: instantiate");
        // let exports = WebAssembly::Module::exports(&module.module);
        // Ok(Self { instance, exports })
        Ok(Self { instance })
    }
}

impl Instance {
    pub fn get_typed_func<Params, Results>(
        &self,
        _store: &mut Store<()>,
        name: &str,
    ) -> Result<TypedFunc<i32, i32>, Error> {
        let exports =
            Reflect::get(&self.instance.as_ref(), &"exports".into()).expect("TODO: get exports");

        let function = Reflect::get(&exports, &name.into())
            .expect("TODO: get function")
            .into();

        Ok(TypedFunc::new(&self.instance, function))
    }
}
