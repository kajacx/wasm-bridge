use crate::preview2::WasiView;
use wasm_bridge::{component::Linker, Result, StoreContextMut};

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:filesystem/types@0.2.0-rc-2023-11-10")?
        .func_wrap(
            "filesystem-error-code",
            |_caller: StoreContextMut<T>, (_index,): (u32,)| {
                // This should be an enum, but it has the same size as an u8
                Ok(Option::<u8>::None)
            },
        )
}
