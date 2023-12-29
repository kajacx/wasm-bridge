use anyhow::bail;
use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use crate::preview2::wasi;
use crate::preview2::WasiView;

// wasm_bridge::component::bindgen!({
//     path: "src/preview2/wits/exit.wit",
//     world: "exports"
// });

impl<T: WasiView + 'static> wasi::cli::exit::Host for T {
    fn exit(&mut self, status: Result<(), ()>) -> Result<()> {
        match status {
            Ok(()) => Ok(()),
            Err(()) => bail!("Guest called exit"),
        }
    }
}

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    wasi::cli::exit::add_to_linker(linker, |d| d)
}
