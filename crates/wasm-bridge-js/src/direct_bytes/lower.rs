use super::{next_multiple_of, ByteBuffer, SizeDescription};
use crate::Result;
use wasm_bindgen::JsValue;

pub trait Lower: SizeDescription {
    /// Gets the "final" thing that is passed into the wasm function call
    fn to_abi<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M);

    /// Writes itself and all children into the memory buffer. Caller flushes the buffer.
    /// This MUST write (or skip) exactly `Self::flat_byte_size()` bytes into the buffer.
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
