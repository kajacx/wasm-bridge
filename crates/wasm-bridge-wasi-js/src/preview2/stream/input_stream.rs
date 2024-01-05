use anyhow::Context;
use wasm_bridge::{component::Linker, Result, StoreContextMut};

use super::{StreamError, StreamResult};
use crate::preview2::WasiView;

pub trait HostInputStream: Send {
    fn read(&mut self, size: usize) -> StreamResult<bytes::Bytes>;
}

pub trait StdinStream: Send {
    fn stream(&self) -> Box<dyn HostInputStream>;

    fn isatty(&self) -> bool;
}

struct VoidStream;

impl HostInputStream for VoidStream {
    fn read(&mut self, _size: usize) -> StreamResult<bytes::Bytes> {
        StreamResult::Err(StreamError::Closed)
    }
}

impl StdinStream for VoidStream {
    fn stream(&self) -> Box<dyn HostInputStream> {
        Box::new(VoidStream)
    }

    fn isatty(&self) -> bool {
        false
    }
}

pub(crate) fn void_stream() -> impl StdinStream {
    VoidStream
}

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:cli/stdin@0.2.0-rc-2023-11-10")?
        .func_wrap("get-stdin", |mut caller: StoreContextMut<T>, (): ()| {
            wasm_bridge::helpers::console_log("CALLING GET STDIN");
            let stream = caller.data().ctx().stdin().stream();
            let index = caller.data_mut().table_mut().input_streams.insert(stream);
            Ok(index)
        })?;

    linker
        .instance("wasi:io/streams@0.2.0-rc-2023-11-10")?
        .func_wrap(
            "[method]input-stream.blocking-read",
            |mut caller: StoreContextMut<T>, (index, len): (u32, u64)| {
                wasm_bridge::helpers::console_log("CALLING INPUT STREAM BLOCKING READ");
                let stream = caller
                    .data_mut()
                    .table_mut()
                    .input_streams
                    .get_mut(index)
                    .context("Get input stream resource")?;

                let result = stream.read(len as usize);

                Ok(result.map(|bytes| bytes.to_vec()))
            },
        )?;

    Ok(())
}
