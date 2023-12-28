use anyhow::bail;
use wasm_bridge::{component::Linker, Result};

use crate::preview2::WasiView;

pub trait InputStream: Send {
    fn as_any(&self) -> &dyn std::any::Any;

    fn readable(&self) -> Result<()>;

    fn read(&mut self, buf: &mut [u8]) -> Result<(u64, bool)>;

    fn num_ready_bytes(&self) -> Result<u64>;
}

struct VoidStream;

impl InputStream for VoidStream {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn readable(&self) -> Result<()> {
        Ok(())
    }

    fn read(&mut self, _buf: &mut [u8]) -> Result<(u64, bool)> {
        Ok((0, true))
    }

    fn num_ready_bytes(&self) -> Result<u64> {
        Ok(0)
    }
}

pub(crate) fn void_stream() -> impl InputStream {
    VoidStream
}

wasm_bridge::component::bindgen!({
    path: "src/preview2/wits/input_streams.wit",
    world: "exports"
});

impl<T: WasiView> wasi::io::streams::Host for T {
    fn read(&mut self, stream_id: u32, max_bytes: u64) -> wasm_bridge::Result<(Vec<u8>, bool)> {
        if stream_id != super::STDIN_IDENT {
            bail!("unexpected read stream id: {stream_id}");
        }

        let mut bytes = vec![0u8; max_bytes as usize];
        let (bytes_written, stream_ended) = self.ctx_mut().stdin().read(&mut bytes)?;

        bytes.truncate(bytes_written as _);
        Ok((bytes, stream_ended))
    }
}

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    Exports::add_to_linker(linker, |d| d)
}
