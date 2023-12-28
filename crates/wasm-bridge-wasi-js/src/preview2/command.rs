use anyhow::bail;
use js_sys::Object;

use crate::preview2::{clocks, WasiView};
use wasm_bridge::component::Linker;
use wasm_bridge::{Result, StoreContextMut};

use super::random;

const STDIN_IDENT: u32 = 0;
const STDOUT_IDENT: u32 = 1;
const STDERR_IDENT: u32 = 2;

pub fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    // Default imports
    // TODO: this doesn't work?
    // linker.set_wasi_imports(get_imports());

    // Overrides
    linker.instance("wasi:io/streams")?.func_wrap(
        "read",
        |data: StoreContextMut<T>, (id, max_bytes): (u32, u64)| {
            if id != STDIN_IDENT {
                bail!("unexpected read stream id: {id}");
            }

            let mut bytes = vec![0u8; max_bytes as usize];

            let (bytes_written, stream_ended) = data.ctx_mut().stdin().read(&mut bytes)?;

            bytes.truncate(bytes_written as _);

            Ok((bytes, stream_ended))
        },
    )?;

    linker.instance("wasi:io/streams")?.func_wrap(
        "write",
        |data: StoreContextMut<T>, (id, buffer): (u32, Vec<u8>)| {
            let bytes_written = match id {
                STDOUT_IDENT => data.ctx_mut().stdout().write(&buffer)?,

                STDERR_IDENT => data.ctx_mut().stderr().write(&buffer)?,

                id => bail!("unexpected write stream id: {id}"),
            };
            Ok(bytes_written)
        },
    )?;

    random::add_to_linker(linker)?;
    clocks::add_to_linker(linker)?;

    Ok(())
}
