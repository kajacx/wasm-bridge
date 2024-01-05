use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use super::WasiView;

mod environment;
mod exit;

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    environment::add_to_linker(linker)?;
    exit::add_to_linker(linker)?;

    Ok(())
}
