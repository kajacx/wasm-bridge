// From this PR https://github.com/kajacx/wasm-bridge/pull/3 by zimond

use js_sys::{Reflect, Uint8Array, WebAssembly};

use crate::{helpers::map_js_error, AsContextMut, Result};

#[derive(Clone)]
pub struct Memory {
    memory: WebAssembly::Memory,
}

impl Memory {
    pub(crate) fn new(memory: WebAssembly::Memory) -> Self {
        Self { memory }
    }

    pub fn write(&self, _: impl AsContextMut, offset: usize, buffer: &[u8]) -> Result<()> {
        let memory = Reflect::get(&self.memory, &"buffer".into())
            .map_err(map_js_error("Memory has no buffer field"))?;
        let mem = Uint8Array::new_with_byte_offset_and_length(
            &memory,
            offset as u32,
            buffer.len() as u32,
        );
        mem.copy_from(buffer);
        Ok(())
    }

    pub fn read(&self, _: impl AsContextMut, offset: usize, buffer: &mut [u8]) -> Result<()> {
        let memory = Reflect::get(&self.memory, &"buffer".into())
            .map_err(map_js_error("Memory has no buffer field"))?;
        let mem = Uint8Array::new_with_byte_offset_and_length(
            &memory,
            offset as u32,
            buffer.len() as u32,
        );
        mem.copy_to(buffer);
        Ok(())
    }
}
