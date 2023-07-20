use js_sys::{Function, Object};
use wasm_bindgen::JsValue;

use super::*;
use crate::{Result, Store, ToJsValue};

static JCO_BYTES: &'static [u8] = include_bytes!("../../../../resources/transformed/jco-web.zip");
static WASI_IMPORTS: &'static str =
    include_str!("../../../../resources/transformed/preview2-shim/bundled.js");

pub struct ComponentLoader {
    // From the jco wasm file
    generate: Function,
}

impl ComponentLoader {
    // TODO: this should not really be a "result" ... add a unit test?
    pub fn new() -> Result<Self> {
        // TODO: using a different store ... should not matter
        let mut store = Store::<()>::default();
        let component = Component::from_zip(JCO_BYTES)?;

        let mut linker = Linker::new(store.engine());
        linker.set_wasi_imports(Self::get_wasi_imports());

        let instance = linker.instantiate(&mut store, &component)?;

        let func = instance
            .exports(&mut store)
            .root()
            .typed_func::<(), ()>("generate")?
            .func()
            .clone();

        Ok(Self {
            generate: func.function,
        })
    }

    pub fn compile_component(self, bytes: &[u8]) -> Result<Component> {
        let compile_fn = Self::get_compilation_fn();
        let bytes_js = bytes.to_js_value();

        let files = compile_fn
            .call2(&JsValue::UNDEFINED, &self.generate, &bytes_js)
            .expect("call compile_fn");

        crate::helpers::log_js_value("files", &files);

        todo!("how are files?")
    }

    fn get_wasi_imports() -> Object {
        js_sys::eval(WASI_IMPORTS)
            .expect("should get wasi imports from file")
            .into()
    }

    fn get_compilation_fn() -> Function {
        js_sys::eval(
            r#"
        (generate, bytes) => {
            let opts = {
                name: "component",
                map: [],
                tlaCompat: false,
                instantiation: true,
                base64Cutoff: 4096,
            };
            let { files } = generate(bytes, opts);
            return files;
        }
        "#,
        )
        .unwrap()
        .into()
    }
}
