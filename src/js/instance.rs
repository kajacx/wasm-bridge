use crate::*;
use js_sys::{Array, Object, Reflect, WebAssembly};

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
    pub fn get_typed_func(
        &self,
        _store: &mut Store<()>,
        name: &str,
    ) -> Result<TypedFunc<i32, i32>, Error> {
        console_log::init_with_level(log::Level::Debug).unwrap();
        log::info!("Hello?");
        // self.exports.iter().for_each(|x| log::info!("{x:?}"));

        // let function = self.exports.iter().find(|obj| {
        //     let obj_name = Reflect::get(obj.as_ref(), &"name".into()).unwrap();
        //     let obj_kind = Reflect::get(obj.as_ref(), &"kind".into()).unwrap();
        //     obj_name == name && obj_kind == "function"
        // });

        let exports =
            Reflect::get(&self.instance.as_ref(), &"exports".into()).expect("TODO: get exports");
        let function = Reflect::get(&exports, &name.into())
            .expect("TODO: get function")
            .into();

        log::info!("function is: {function:?}");

        Ok(TypedFunc::new(&self.instance, function))

        // match function {
        //     Some(_) => Ok(TypedFunc::new()),
        //     None => Err(Error),
        // }
    }
}
