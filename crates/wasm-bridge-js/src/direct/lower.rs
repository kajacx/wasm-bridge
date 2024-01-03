use super::{ByteBuffer, SizeDescription};
use crate::{Result, ToJsValue};
use js_sys::{Array, Reflect};
use wasm_bindgen::JsValue;

pub trait Lower: SizeDescription {
    /// Serializes self to JS arguments to be passes to an exported method.
    fn to_js_args<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) -> Result<()>;

    /// Converts self from a return from an imported function to a JS return value.
    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue>;

    /// Writes itself and all children into the memory buffer. Caller flushes the buffer.
    /// This MUST write (or skip) exactly `Self::BYTE_SIZE` bytes into the buffer.
    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()>;

    /// Returns this as a pointer to the data. Must be used if NUM_ARGS > 1
    fn to_js_ptr_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue> {
        debug_assert!(Self::NUM_ARGS > 1);

        let mut buffer = memory.allocate(Self::ALIGNMENT, Self::BYTE_SIZE)?;
        self.write_to(&mut buffer, memory)?;

        let addr = memory.flush(buffer) as u32;
        Ok(addr.to_js_value())
    }
}

pub trait WriteableMemory {
    /// Allocates `size` bytes with `align` alignment.
    fn allocate(&self, align: usize, size: usize) -> Result<ByteBuffer>;

    /// Actually writes the slice into memory, returning the slice's address.
    fn flush(&self, buffer: ByteBuffer) -> usize;
}

impl<T: WriteableMemory> WriteableMemory for &T {
    fn allocate(&self, align: usize, size: usize) -> Result<ByteBuffer> {
        T::allocate(self, align, size)
    }

    fn flush(&self, buffer: ByteBuffer) -> usize {
        T::flush(self, buffer)
    }
}

pub struct JsArgsWriter {
    args: Array,
    index: u32,
}

impl JsArgsWriter {
    pub fn new(num_args: u32) -> Self {
        Self {
            args: Array::new_with_length(num_args),
            index: 0,
        }
    }

    pub fn push(&mut self, arg: &JsValue) {
        Reflect::set_u32(&self.args, self.index, value);
        self.index += 1;
    }
}
