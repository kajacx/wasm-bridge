use anyhow::bail;
use js_sys::Function;
use wasm_bindgen::JsValue;

use wasm_bridge::{component::Linker, Result};

use crate::preview2::WasiView;

pub trait OutputStream: Send {
    fn as_any(&self) -> &dyn std::any::Any;

    fn writable(&self) -> Result<()>;

    fn write(&mut self, buf: &[u8]) -> Result<u64>;
}

struct VoidingStream;

impl OutputStream for VoidingStream {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn writable(&self) -> Result<()> {
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> Result<u64> {
        Ok(buf.len() as _)
    }
}

pub(crate) fn voiding_stream() -> impl OutputStream {
    VoidingStream
}

struct InheritStream(String);

impl OutputStream for InheritStream {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn writable(&self) -> Result<()> {
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> Result<u64> {
        let text = String::from_utf8_lossy(buf);

        // Do not store the js Function, it makes the stream not Send
        let function: Function = js_sys::eval(&self.0)
            .expect("TODO: user error: Eval inherit stream function")
            .into();
        debug_assert!(function.is_function());

        function
            .call1(&JsValue::UNDEFINED, &text.as_ref().into())
            .expect("TODO: user error: Call output stream function");

        Ok(buf.len() as _)
    }
}

pub(crate) fn console_log_stream() -> impl OutputStream {
    InheritStream("console.log".into())
}

pub(crate) fn console_error_stream() -> impl OutputStream {
    InheritStream("console.error".into())
}

wasm_bridge::component::bindgen!({
    path: "src/preview2/wits/output_streams.wit",
    world: "exports"
});

impl<T: WasiView> wasi::io::streams::Host for T {
    fn write(&mut self, stream_id: u32, bytes: Vec<u8>) -> Result<u64> {
        let bytes_written = match stream_id {
            STDOUT_IDENT => self.ctx_mut().stdout().write(&bytes)?,
            STDERR_IDENT => self.ctx_mut().stderr().write(&bytes)?,
            id => bail!("unexpected write stream id: {id}"),
        };
        Ok(bytes_written)
    }
}

pub(crate) fn add_to_linker<T: WasiView>(linker: &mut Linker<T>) -> Result<()> {
    Exports::add_to_linker(linker, |d| d)
}
