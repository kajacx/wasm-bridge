use wasm_bridge::{component::Linker, Result};

use super::WasiView;

mod preopens;
mod types;

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    preopens::add_to_linker(linker)?;
    types::add_to_linker(linker)?;
    Ok(())
}
