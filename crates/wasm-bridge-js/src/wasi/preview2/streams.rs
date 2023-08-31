use anyhow::bail;

use crate::{component::Linker, Result, StoreContextMut};

use super::{stdio::STDIN_IDENT, WasiView};

// See: <https://github.com/WebAssembly/wasi-io/blob/main/wit/streams.wit>
pub(crate) fn add_to_linker<T: 'static + WasiView>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:io/streams")?
        .func_wrap(
            "read",
            |data: StoreContextMut<T>,
             (id, max_bytes): (u32, u64)|
             -> Result<Result<(Vec<u8>, u32), ()>> {
                if id != STDIN_IDENT {
                    bail!("unexpected read stream id: {id}")
                }

                let mut bytes = vec![0u8; max_bytes as usize];

                let (bytes_written, status) = data.ctx_mut().stdin().read(&mut bytes)?;

                bytes.truncate(bytes_written as _);

                Ok(Ok((bytes, status as u32)))
            },
        )?
        .func_wrap(
            "write",
            |data: StoreContextMut<T>,
             (id, buffer): (u32, Vec<u8>)|
             -> Result<Result<(u64, u32), ()>> {
                let (bytes_written, status) = match id {
                    STDOUT_IDENT => data.ctx_mut().stdout().write(&buffer)?,

                    STDERR_IDENT => data.ctx_mut().stderr().write(&buffer)?,

                    id => bail!("unexpected write stream id: {id}"),
                };

                Ok(Ok((bytes_written as u64, status as u32)))
            },
        )?;

    Ok(())
}
