use anyhow::Context;
use js_sys::{Array, Function};
use wasm_bindgen::JsValue;

use crate::{helpers::map_js_error, Result};

use super::{Lower, ReadableMemory, WriteableMemory};

#[derive(Debug, Clone)]
pub struct ModuleMemoryInner {
    pub(crate) memory: crate::Memory,
    pub(crate) realloc: Function,
}

// #[derive(Debug, Clone)]
pub type ModuleMemory = ModuleMemoryInner;

// impl ModuleMemory {
//     pub(crate) fn new(memory: crate::Memory, realloc: Function) -> Self {
//         Self(ModuleMemoryInner::new(memory, realloc))
//     }
// }

impl ModuleMemoryInner {
    pub(crate) fn new(memory: crate::Memory, realloc: Function) -> Self {
        Self { memory, realloc }
    }

    fn malloc(&self, align: usize, size: usize) -> Result<usize> {
        thread_local! {
            static ARGS: Array = {
                let array = Array::new_with_length(4);
                array.set(0, 0.into());
                array.set(1, 0.into());
                array
            };
        }

        ARGS.with(|args| {
            args.set(2, (align as u32).into());
            args.set(3, (size as u32).into());

            let result = self
                .realloc
                .apply(&JsValue::UNDEFINED, args)
                .map_err(map_js_error("call capi_realloc"))?;

            Ok(result.as_f64().context("realloc should return a number")? as usize)
        })
    }
}

impl WriteableMemory for ModuleMemory {
    fn allocate(&self, align: usize, size: usize) -> Result<ByteBuffer> {
        let address = self.malloc(align, size)?;
        Ok(ByteBuffer::new(address, size))
    }

    fn flush(&self, slice: ByteBuffer) -> usize {
        self.memory
            .write_impl(slice.address, &slice.data)
            .expect("write bytes to buffer"); // TODO: Can this fail? If so, why?

        slice.address
    }
}

impl ReadableMemory for ModuleMemory {
    fn read_to_slice(&self, addr: usize, target: &mut [u8]) {
        self.memory
            .read_impl(addr, target)
            .expect("read bytes from memory")
    }
}

pub struct ByteBuffer {
    address: usize,
    data: Vec<u8>,
}

impl ByteBuffer {
    pub fn new(address: usize, size: usize) -> Self {
        Self {
            address,
            data: Vec::with_capacity(size as _),
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn skip(&mut self, num_bytes: usize) {
        // TODO: could just skip instead of writing 0s with some unsafe magic
        self.data.extend((0..num_bytes).map(|_| 0));
    }

    pub fn write<T: Lower, M: WriteableMemory>(&mut self, value: &T, memory: &M) -> Result<()> {
        value.write_to(self, memory)
    }
}
