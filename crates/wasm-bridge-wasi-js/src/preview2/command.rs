use crate::preview2::{clocks, WasiView};
use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use super::{cli, random, stream};

pub fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker.set_wasi_object(get_wasi_object);

    stream::add_to_linker(linker)?;
    random::add_to_linker(linker)?;
    clocks::add_to_linker(linker)?;
    cli::add_to_linker(linker)?;

    Ok(())
}

fn get_wasi_object() -> wasm_bridge::js_sys::Object {
    let shim = include_str!("../../wasi-shim.js");
    let shim = wasm_bridge::js_sys::eval(shim).expect("exec shim");
    shim.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_get_wasi_object() {
        let object = get_wasi_object();
        assert!(object.is_object());
    }
}
