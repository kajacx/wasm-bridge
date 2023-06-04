use crate::*;
use js_sys::{Array, Object, WebAssembly};

pub struct Instance {
    instance: WebAssembly::Instance,
    exports: Array,
}

impl Instance {
    pub fn new(
        _store: &mut Store<()>,
        module: &Module,
        _: impl AsRef<[()]>,
    ) -> Result<Self, Error> {
        let imports = Object::new();
        let instance = WebAssembly::Instance::new(&module.module, &imports).expect("TODO");
        let exports = WebAssembly::Module::exports(&module.module);
        Ok(Self { instance, exports })
    }
}

impl Instance {
    pub fn get_typed_func<Params, Results>(
        &self,
        _store: &mut Store<()>,
        _name: &str,
    ) -> Result<TypedFunc<Params, Results>, Error> {
        println!("Hello?");
        self.exports.iter().for_each(|x| println!("{x:?}"));
        todo!()
    }
}
