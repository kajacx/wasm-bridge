use js_sys::Object;

use crate::component::Linker;
use crate::Result;

static WASI_IMPORTS_STR: &str =
    include_str!("../../../../../../resources/transformed/preview2-shim/bundled.js");

pub fn add_to_linker<T>(linker: &mut Linker<T>) -> Result<()> {
    linker.set_wasi_imports(get_imports());
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
