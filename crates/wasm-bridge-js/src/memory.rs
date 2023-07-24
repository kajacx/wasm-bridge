use js_sys::{Reflect, Uint8Array, WebAssembly};

use crate::{helpers::map_js_error, AsContextMut, Error};

#[derive(Clone)]
pub struct Memory {
    pub(crate) memory: WebAssembly::Memory,
}

impl Memory {
    pub fn write(&self, _: impl AsContextMut, offset: usize, buffer: &[u8]) -> Result<(), Error> {
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

    pub fn read(
        &self,
        _: impl AsContextMut,
        offset: usize,
        buffer: &mut [u8],
    ) -> Result<(), Error> {
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
