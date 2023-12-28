use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use crate::preview2::{WasiView, STDIN_IDENT};

wasm_bridge::component::bindgen!({
    path: "src/preview2/wits/stdin.wit",
    world: "exports"
});

impl<T: WasiView> wasi::cli::stdin::Host for T {
    fn get_stdin(&mut self) -> Result<u32> {
        Ok(STDIN_IDENT)
    }
}

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    Exports::add_to_linker(linker, |d| d)
}
