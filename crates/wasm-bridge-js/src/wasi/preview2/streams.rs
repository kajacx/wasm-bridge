use anyhow::bail;

use crate::{component::Linker, Result, StoreContextMut};

use super::{
    stdio::{STDERR_IDENT, STDIN_IDENT, STDOUT_IDENT},
    WasiView,
};

// See: <https://github.com/WebAssembly/wasi-io/blob/main/wit/streams.wit>
pub(crate) fn add_to_linker<T: 'static + WasiView>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:io/streams")?
        .func_wrap(
            "read",
            |data: StoreContextMut<T>,
             (id, max_bytes): (u32, u64)|
             -> Result<Result<(Vec<u8>, String), ()>> {
                // tracing::debug!(?id, ?max_bytes, "read");
                if id != STDIN_IDENT {
                    bail!("unexpected read stream id: {id}")
                }

                let mut bytes = vec![0u8; max_bytes as usize];

                let (count, status) = data.ctx_mut().stdin().read(&mut bytes)?;
                // tracing::debug!(?count, "bytes read");

                bytes.truncate(count as _);

                Ok(Ok((bytes, status.to_variant())))
            },
        )?
        .func_wrap(
            "blocking-read",
            |data: StoreContextMut<T>,
             (id, max_bytes): (u32, u64)|
             -> Result<Result<(Vec<u8>, String), ()>> {
                tracing::debug!(?id, ?max_bytes, "blocking-read");
                if id != STDIN_IDENT {
                    bail!("unexpected read stream id: {id}")
                }

                let mut bytes = vec![0u8; max_bytes as usize];

                let (bytes_written, status) = data.ctx_mut().stdin().read(&mut bytes)?;

                bytes.truncate(bytes_written as _);

                Ok(Ok((bytes, status.to_variant())))
            },
        )?
        .func_wrap(
            "write",
            |data: StoreContextMut<T>,
             (id, buffer): (u32, Vec<u8>)|
             -> Result<Result<(u64, String), ()>> {
                tracing::debug!(?id, "write");
                let (bytes_written, status) = match id {
                    STDOUT_IDENT => data.ctx_mut().stdout().write(&buffer)?,

                    STDERR_IDENT => data.ctx_mut().stderr().write(&buffer)?,

                    id => bail!("unexpected write stream id: {id}"),
                };

                Ok(Ok((bytes_written as u64, status.to_variant())))
            },
        )?
        // blocking write has the same signature, thank god
        .func_wrap(
            "blocking-write",
            |data: StoreContextMut<T>,
             (id, buffer): (u32, Vec<u8>)|
             -> Result<Result<(u64, String), ()>> {
                tracing::debug!(?id, "blocking-write");
                let (bytes_written, status) = match id {
                    STDOUT_IDENT => data.ctx_mut().stdout().write(&buffer)?,

                    STDERR_IDENT => data.ctx_mut().stderr().write(&buffer)?,

                    id => bail!("unexpected write stream id: {id}"),
                };

                Ok(Ok((bytes_written as u64, status.to_variant())))
            },
        )?;

    Ok(())
}
