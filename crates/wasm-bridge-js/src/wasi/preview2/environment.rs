//! Provides environment interaction for WASI.
//!
//! See: <https://github.com/WebAssembly/wasi-cli/blob/main/wit/environment.wit>

use crate::{component::Linker, Result, StoreContextMut};

use super::WasiView;

/// Adds environment integration to the linker
pub(crate) fn add_to_linker<T: 'static + WasiView>(linker: &mut Linker<T>) -> Result<()> {
    linker.instance("wasi:cli-base/environment")?.func_wrap(
        "get-environment",
        |data: StoreContextMut<T>, (): ()| {
            Ok(data
                .ctx()
                .environment()
                .iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect::<Vec<_>>())
        },
    )?;

    linker.instance("wasi:cli-base/environment")?.func_wrap(
        "get-arguments",
        |_data: StoreContextMut<T>, (): ()| -> Result<String> { unimplemented!() },
    )?;

    linker
        .instance("wasi:cli-base/environment")?
        .func_wrap("initial-cwd", |_data: StoreContextMut<T>, (): ()| {
            Ok(String::from("."))
        })?;

    Ok(())
}
