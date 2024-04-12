use wasm_bridge::{component::Linker, Result, StoreContextMut};

use crate::js::WasiView;

// TODO: drop error properly
pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:io/error@0.2.0-rc-2023-11-10")?
        .func_wrap(
            "[resource-drop]error",
            |_caller: StoreContextMut<T>, (_index,): (u32,)| Ok(()),
        )
}
