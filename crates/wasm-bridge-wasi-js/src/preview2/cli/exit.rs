use anyhow::bail;
use wasm_bridge::component::Linker;
use wasm_bridge::Result;
use wasm_bridge::StoreContextMut;

use crate::preview2::WasiView;

// TODO: implement and test exit properly
pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:cli/exit@0.2.0-rc-2023-11-10")?
        .func_wrap(
            "exit",
            |caller: StoreContextMut<T>, (status,): (Result<(), ()>,)| Ok(()),
        )
}
