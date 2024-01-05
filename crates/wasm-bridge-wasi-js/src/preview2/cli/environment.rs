use wasm_bridge::component::Linker;
use wasm_bridge::{Result, StoreContextMut};

use crate::preview2::WasiView;

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:cli/environment@0.2.0-rc-2023-11-10")?
        .func_wrap("get-environment", |_caller: StoreContextMut<T>, (): ()| {
            // TODO: should be a vec of string tuples, but it's empty, so it doesn't matter
            Ok(Vec::<u32>::new())
        })
}
