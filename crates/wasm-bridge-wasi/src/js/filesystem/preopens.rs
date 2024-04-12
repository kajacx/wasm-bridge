use crate::js::WasiView;
use wasm_bridge::{component::Linker, Result, StoreContextMut};

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:filesystem/preopens@0.2.0-rc-2023-11-10")?
        .func_wrap("get-directories", |_caller: StoreContextMut<T>, (): ()| {
            // This should be a vec of tuples of descriptors and names, but it's empty, so it doesn't matter
            Ok(Vec::<u32>::new())
        })
}
