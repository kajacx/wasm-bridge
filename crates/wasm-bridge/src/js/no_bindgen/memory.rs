// From this PR https://github.com/kajacx/wasm-bridge/pull/3 by zimond

use js_sys::{Reflect, Uint8Array, WebAssembly};
use wasm_bindgen::JsValue;

use crate::{helpers::map_js_error, AsContextMut, Result};

#[derive(Clone, Debug)]
pub struct Memory {
    pub(crate) memory: WebAssembly::Memory,
}

impl Memory {
    pub(crate) fn new(memory: WebAssembly::Memory) -> Self {
        Self { memory }
    }

    // We need this for compatible signature with wasmtime
    pub fn write(&self, _: impl AsContextMut, offset: usize, buffer: &[u8]) -> Result<()> {
        self.write_impl(offset, buffer)
    }

    pub(crate) fn write_impl(&self, offset: usize, buffer: &[u8]) -> Result<()> {
        let memory = self.get_memory_buffer()?;

        let mem = Uint8Array::new_with_byte_offset_and_length(
            &memory,
            offset as u32,
            buffer.len() as u32,
        );
        mem.copy_from(buffer);

        Ok(())
    }

    // We need this for compatible signature with wasmtime
    pub fn read(&self, _: impl AsContextMut, offset: usize, buffer: &mut [u8]) -> Result<()> {
        self.read_impl(offset, buffer)
    }

    pub(crate) fn read_impl(&self, offset: usize, buffer: &mut [u8]) -> Result<()> {
        let memory = self.get_memory_buffer()?;

        let mem = Uint8Array::new_with_byte_offset_and_length(
            &memory,
            offset as u32,
            buffer.len() as u32,
        );
        mem.copy_to(buffer);

        Ok(())
    }

    fn get_memory_buffer(&self) -> Result<JsValue> {
        thread_local! {
            static BUFFER_NAME: JsValue = "buffer".into();
        }

        BUFFER_NAME.with(|buffer_name| {
            Reflect::get(&self.memory, buffer_name)
                .map_err(map_js_error("Memory has no buffer field"))
        })
    }
}
