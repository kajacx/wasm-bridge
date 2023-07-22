use js_sys::Object;

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
        |data: StoreContextMut<T>, (id, buffer): (u32, Vec<u8>)| {
            let bytes_written = match id {
                STDOUT_IDENT => data
                    .ctx_mut()
                    .stdout()
                    .write(&buffer)
                    .expect("write to out"),

                STDERR_IDENT => data
                    .ctx_mut()
                    .stderr()
                    .write(&buffer)
                    .expect("write to err"),

                _ => panic!("TODO: unexpected stream id"),
            };

            Ok(bytes_written)
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
