use anyhow::Context;
use js_sys::Function;
use wasm_bindgen::JsValue;

use wasm_bridge::StoreContextMut;
use wasm_bridge::{component::Linker, Result};

use super::{StreamResult, Subscribe};
use crate::js::WasiView;

// TODO: This seems to need to be "Send" on JS but not on sys, why?
pub trait HostOutputStream: Subscribe + Send {
    fn write(&mut self, bytes: bytes::Bytes) -> StreamResult<()>;

    fn flush(&mut self) -> StreamResult<()>;

    fn check_write(&mut self) -> StreamResult<usize>;
}

pub trait StdoutStream: Send {
    fn stream(&self) -> Box<dyn HostOutputStream>;

    fn isatty(&self) -> bool;
}

struct VoidingStream;

impl Subscribe for VoidingStream {
    fn ready(&mut self) {}
}

impl HostOutputStream for VoidingStream {
    fn write(&mut self, _bytes: bytes::Bytes) -> StreamResult<()> {
        Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        Ok(usize::MAX)
    }
}

impl StdoutStream for VoidingStream {
    fn stream(&self) -> Box<dyn HostOutputStream> {
        Box::new(VoidingStream)
    }

    fn isatty(&self) -> bool {
        false
    }
}

pub(crate) fn voiding_stream() -> impl StdoutStream {
    VoidingStream
}

#[derive(Debug, Clone)]
enum WhichOut {
    StdOut,
    StdErr,
}

#[derive(Debug, Clone)]
struct InheritStream(WhichOut);

impl Subscribe for InheritStream {
    fn ready(&mut self) {}
}

impl HostOutputStream for InheritStream {
    fn write(&mut self, bytes: bytes::Bytes) -> StreamResult<()> {
        let text = String::from_utf8_lossy(&bytes);

        // Do not store the js Function, it makes the stream not Send
        let function: Function = js_sys::eval(match self.0 {
            WhichOut::StdOut => "console.log",
            WhichOut::StdErr => "console.error",
        })
        .expect("eval console.log or console.error")
        .into();

        function
            .call1(&JsValue::UNDEFINED, &text.as_ref().into())
            .expect("call console.log or console.error");

        Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        Ok(usize::MAX)
    }
}

impl StdoutStream for InheritStream {
    fn stream(&self) -> Box<dyn HostOutputStream> {
        Box::new(self.clone())
    }

    fn isatty(&self) -> bool {
        false
    }
}

pub(crate) fn console_log_stream() -> impl StdoutStream {
    InheritStream(WhichOut::StdOut)
}

pub(crate) fn console_error_stream() -> impl StdoutStream {
    InheritStream(WhichOut::StdErr)
}

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:cli/stdout@0.2.0-rc-2023-11-10")?
        .func_wrap("get-stdout", |mut caller: StoreContextMut<T>, (): ()| {
            let mut stream = caller.data_mut().ctx().stdout().stream();
            stream.ready();
            let index = caller.data_mut().table().output_streams.insert(stream);
            Ok(index)
        })?;

    linker
        .instance("wasi:cli/stderr@0.2.0-rc-2023-11-10")?
        .func_wrap("get-stderr", |mut caller: StoreContextMut<T>, (): ()| {
            let mut stream = caller.data_mut().ctx().stderr().stream();
            stream.ready();
            let index = caller.data_mut().table().output_streams.insert(stream);
            Ok(index)
        })?;

    linker
        .instance("wasi:io/streams@0.2.0-rc-2023-11-10")?
        .func_wrap(
            "[method]output-stream.blocking-write-and-flush",
            |mut caller: StoreContextMut<T>, (index, bytes): (u32, Vec<u8>)| {
                let stream = caller
                    .data_mut()
                    .table()
                    .output_streams
                    .get_mut(index)
                    .context("Get output stream resource")?;

                let mut start = 0;
                while start < bytes.len() {
                    let max_size = match stream.check_write() {
                        Ok(size) => size,
                        Err(err) => return Ok(Err(err)),
                    };
                    let end = usize::min(start + max_size, bytes.len());
                    let result = stream.write(bytes::Bytes::copy_from_slice(&bytes[start..end]));
                    if result.is_err() {
                        return Ok(result);
                    }
                    start = end;
                }

                Ok(Ok(()))
            },
        )?;

    Ok(())
}
