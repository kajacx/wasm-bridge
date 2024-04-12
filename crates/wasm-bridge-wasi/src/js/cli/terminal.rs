use wasm_bridge::{component::Linker, Result, StoreContextMut};

use crate::js::WasiView;

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:cli/terminal-stdin@0.2.0-rc-2023-11-10")?
        .func_wrap(
            "get-terminal-stdin",
            |_caller: StoreContextMut<T>, (): ()| Ok(Option::<u32>::None),
        )?;

    linker
        .instance("wasi:cli/terminal-stdout@0.2.0-rc-2023-11-10")?
        .func_wrap(
            "get-terminal-stdout",
            |_caller: StoreContextMut<T>, (): ()| Ok(Option::<u32>::None),
        )?;

    linker
        .instance("wasi:cli/terminal-stderr@0.2.0-rc-2023-11-10")?
        .func_wrap(
            "get-terminal-stderr",
            |_caller: StoreContextMut<T>, (): ()| Ok(Option::<u32>::None),
        )?;

    Ok(())
}
