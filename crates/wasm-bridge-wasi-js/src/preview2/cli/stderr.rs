use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use crate::preview2::{WasiView, STDERR_IDENT};

wasm_bridge::component::bindgen!({
    path: "src/preview2/wits/stderr.wit",
    world: "exports"
});

impl<T: WasiView> wasi::cli::stderr::Host for T {
    fn get_stderr(&mut self) -> Result<u32> {
        Ok(STDERR_IDENT)
    }
}

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    Exports::add_to_linker(linker, |d| d)
}
