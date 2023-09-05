use crate::{component::Linker, Result, StoreContextMut};

use super::WasiView;

pub(crate) fn add_to_linker<T: 'static + WasiView>(linker: &mut Linker<T>) -> Result<()> {
    linker.instance("wasi:filesystem/preopens")?.func_wrap(
        "get-directories",
        |_data: StoreContextMut<T>, (): ()| -> Result<Vec<(u32, String)>> { Ok(vec![]) },
    )?;

    Ok(())
}
