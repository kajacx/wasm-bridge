use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use crate::preview2::WasiView;

wasm_bridge::component::bindgen!({
    path: "src/preview2/wits/terminal_stderr.wit",
    world: "exports"
});

impl<T: WasiView> wasi::cli::terminal_stderr::Host for T {
    fn get_terminal_stderr(&mut self) -> Result<()> {
        Ok(())
    }
}

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    Exports::add_to_linker(linker, |d| d)
}
