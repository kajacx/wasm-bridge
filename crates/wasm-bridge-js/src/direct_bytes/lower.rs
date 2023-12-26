use super::{ByteBuffer, SizeDescription};
use crate::Result;
use wasm_bindgen::JsValue;

pub trait Lower: SizeDescription {
    /// Serializes self to JS arguments to be passes to an exported method.
    fn to_js_args<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) -> Result<()>;

    /// Converts self from a return from an imported function to a JS return value.
    fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<JsValue>;

    /// Writes itself and all children into the memory buffer. Caller flushes the buffer.
    /// This MUST write (or skip) exactly `Self::FLAT_BYTE_SIZE` bytes into the buffer.
    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()>;
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
