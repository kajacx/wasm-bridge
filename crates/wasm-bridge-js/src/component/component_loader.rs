use anyhow::Context;
use js_sys::{Function, Object, Reflect, Uint8Array};
use wasm_bindgen::JsValue;

use super::*;
use crate::{helpers::map_js_error, Result, Store, ToJsValue};

static JCO_BYTES: &[u8] = include_bytes!("../../../../resources/transformed/jco-web.zip");
static WASI_IMPORTS: &str =
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

        let files_js = compile_fn
            .call2(&JsValue::UNDEFINED, &self.generate, &bytes_js)
            .expect("call compile_fn");

        let files_js =
            Object::from_entries(&files_js).map_err(map_js_error("Files object from entries"))?;

        let names = Object::get_own_property_names(&files_js);

        let length = names.length();
        let mut files = Vec::with_capacity(length as _);

        for index in 0..length {
            let name_js =
                Reflect::get_u32(&names, index).map_err(map_js_error("Get file from generate"))?;

            let mut name = name_js.as_string().context("Property name is string")?;

            let bytes_js =
                Reflect::get(&files_js, &name_js).map_err(map_js_error("Get object property"))?;

            let bytes_js = Uint8Array::from(bytes_js);

            let mut bytes = bytes_js.to_vec();

            if name.ends_with(".js") {
                name = "sync_component.js".into();
                let text = String::from_utf8(bytes)?;
                let text = modify_js(&text);
                bytes = text.into();
            }

            files.push((name, bytes));
        }

        Component::from_files(files)
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

// TODO: duplication with wasm-bridge-cli
fn modify_js(text: &str) -> String {
    // function signature
    let text = text.replace("export async function", "function");

    // remove all awaits
    let text = text.replace("await ", "");

    // remove Promise.all call - not necessary
    // let regex = Regex::new(".*Promise\\.all.*").unwrap();
    // let text = regex.replace_all(&text, "");

    // Final update
    let text = format!("(() => {{\n{text}\nreturn instantiate;\n}})()\n");

    text
}
