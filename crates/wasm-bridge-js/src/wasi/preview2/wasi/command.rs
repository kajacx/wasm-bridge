use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

use crate::component::Linker;
use crate::wasi::preview2::WasiView;
use crate::{Result, StoreContextMut};

static WASI_IMPORTS_STR: &str =
    include_str!("../../../../../../resources/transformed/preview2-shim/bundled.js");

const STDOUT_IDENT: u32 = 1;
const STDERR_IDENT: u32 = 2;

pub fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    // Default imports
    linker.set_wasi_imports(get_imports());

    // Overrides
    linker.instance("wasi:io/streams")?.func_wrap(
        "write",
        |data: StoreContextMut<T>, (id, buffer): (u32, JsValue)| {
            let js_val = match id {
                STDOUT_IDENT => {
                    if let Some(out) = data.ctx().stdout() {
                        out.call1(&JsValue::UNDEFINED, &buffer)
                            .expect("TODO: stdout some")
                    } else {
                        Reflect::get(&buffer, &"byteLength".into()).expect("TODO: stdout none")
                    }
                }
                STDERR_IDENT => {
                    if let Some(err) = data.ctx().stderr() {
                        err.call1(&JsValue::UNDEFINED, &buffer)
                            .expect("TODO: stderr some")
                    } else {
                        Reflect::get(&buffer, &"byteLength".into()).expect("TODO: stderr none")
                    }
                }
                _ => panic!("TODO: unexpected stream id"),
            };

            Ok(i64::try_from(js_val).unwrap_or_else(|val| {
                val.as_f64().expect("TODO: must be number if not bigint") as i64
            }))
        },
    )?;

    Ok(())
}

fn get_imports() -> Object {
    let imports = js_sys::eval(WASI_IMPORTS_STR).expect("eval bundled wasi imports");

    assert!(imports.is_object(), "wasi imports must be an object");

    imports.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn should_get_imports() {
        let _ = get_imports();
    }
}
