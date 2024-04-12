use wasm_bridge::{component::Linker, Result, StoreContextMut};

use crate::js::WasiView;

// TODO: implement and test exit properly
pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker.instance("wasi:cli/exit@0.2.0")?.func_wrap(
        "exit",
        |_caller: StoreContextMut<T>, (_status,): (Result<(), ()>,)| Ok(()),
    )
}
