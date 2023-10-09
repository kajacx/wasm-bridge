use std::io;

use wasm_bridge::{component::Linker, Result, StoreContextMut};

use super::WasiView;

pub type Descriptor = u32;
pub type OutputStream = u32;
pub type InputStream = u32;

pub(crate) fn add_to_linker<T: 'static + WasiView>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:filesystem/types")?
        .func_wrap(
            "append-via-stream",
            |_data: StoreContextMut<T>, (_this,): (Descriptor,)| -> Result<OutputStream> {
                Err(io::Error::new(io::ErrorKind::Unsupported, "append-via-stream").into())
            },
        )?
        .func_wrap(
            "drop-descriptor",
            |_data: StoreContextMut<T>, (_this,): (Descriptor,)| -> Result<()> {
                Err(io::Error::new(io::ErrorKind::Unsupported, "drop-descriptor").into())
            },
        )?
        .func_wrap(
            "get-type",
            |_data: StoreContextMut<T>, (_this,): (Descriptor,)| -> Result<u32> {
                Err(io::Error::new(io::ErrorKind::Unsupported, "get-type").into())
            },
        )?
        .func_wrap(
            "read-via-stream",
            |_data: StoreContextMut<T>, (_this, _off): (Descriptor, u64)| -> Result<InputStream> {
                Err(io::Error::new(io::ErrorKind::Unsupported, "read-via-stream").into())
            },
        )?
        .func_wrap(
            "write-via-stream",
            |_data: StoreContextMut<T>, (_this, _off): (Descriptor, u64)| -> Result<OutputStream> {
                Err(io::Error::new(io::ErrorKind::Unsupported, "write-via-stream").into())
            },
        )?;

    Ok(())
}
