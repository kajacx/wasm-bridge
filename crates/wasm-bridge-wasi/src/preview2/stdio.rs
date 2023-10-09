use wasm_bridge::{component::Linker, Result, StoreContextMut};

use super::WasiView;

pub(crate) const STDIN_IDENT: u32 = 0;
pub(crate) const STDOUT_IDENT: u32 = 1;
pub(crate) const STDERR_IDENT: u32 = 2;
pub(crate) fn add_to_linker<T: 'static + WasiView>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:cli/stdout")?
        .func_wrap("get-stdout", |_data: StoreContextMut<T>, (): ()| {
            Ok(STDOUT_IDENT)
        })?;

    linker
        .instance("wasi:cli/stderr")?
        .func_wrap("get-stderr", |_data: StoreContextMut<T>, (): ()| {
            Ok(STDERR_IDENT)
        })?;

    linker
        .instance("wasi:cli/stdin")?
        .func_wrap("get-stdin", |_data: StoreContextMut<T>, (): ()| {
            Ok(STDIN_IDENT)
        })?;

    Ok(())
}
