use std::io;

use crate::{component::Linker, Result, StoreContextMut};

use super::{stream, WasiView};

pub(crate) fn add_to_linker<T: 'static + WasiView>(linker: &mut Linker<T>) -> Result<()> {
    linker.instance("wasi:cli/terminal-input")?.func_wrap(
        "drop-terminal-input",
        |_data: StoreContextMut<T>, (_this,): (u32,)| -> Result<()> { Ok(()) },
    )?;

    linker.instance("wasi:cli/terminal-output")?.func_wrap(
        "drop-terminal-output",
        |_data: StoreContextMut<T>, (_this,): (u32,)| -> Result<()> { Ok(()) },
    )?;

    Ok(())
}
