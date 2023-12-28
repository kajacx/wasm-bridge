use anyhow::bail;
use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use crate::preview2::WasiView;

wasm_bridge::component::bindgen!({
    path: "src/preview2/wits/exit.wit",
    world: "exports"
});

impl<T: WasiView> wasi::cli::exit::Host for T {
    fn exit(&mut self, status: u32) -> Result<()> {
        bail!("Guest called exit with status {status}");
    }
}

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    Exports::add_to_linker(linker, |d| d)
}
