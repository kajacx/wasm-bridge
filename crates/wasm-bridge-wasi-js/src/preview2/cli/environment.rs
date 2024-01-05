use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use crate::preview2::WasiView;

pub(crate) fn add_to_linker<T: WasiView + 'static>(_linker: &mut Linker<T>) -> Result<()> {
    // TODO: environment
    Ok(())
}
