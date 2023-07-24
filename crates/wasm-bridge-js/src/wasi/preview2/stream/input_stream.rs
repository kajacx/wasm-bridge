use crate::Result;

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
