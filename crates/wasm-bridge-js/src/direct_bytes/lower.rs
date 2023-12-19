use js_sys::{Array, Function};
use wasm_bindgen::JsValue;

use super::SizeDescription;

pub trait Lower: SizeDescription {
    type ReturnAbi;

    /// Gets the "final" thing that is passed into the wasm function call
    fn to_return_abi<M: WriteableMemory>(&self, memory: M) -> Self::ReturnAbi;

    /// Writes itself and all children into the memory slice
    fn write_to<M: WriteableMemory>(&self, memory: M, memory_slice: &mut M::Slice);
}

pub trait WriteableMemory {
    type Slice: WriteableMemorySlice;

    /// Allocates `size` bytes with `align` alignment
    fn allocate(&mut self, align: usize, size: usize) -> Self::Slice;

    /// Actually writes the slice into memory, returning the slice's length in bytes
    fn flush(&mut self, slice: Self::Slice) -> usize;
}

impl<T: WriteableMemory> WriteableMemory for &mut T {
    type Slice = T::Slice;

    fn allocate(&mut self, align: usize, size: usize) -> Self::Slice {
        T::allocate(self, align, size)
    }

    fn flush(&mut self, slice: Self::Slice) -> usize {
        T::flush(self, slice)
    }
}

pub trait WriteableMemorySlice {
    fn write(&mut self, bytes: &[u8]);
}

impl<T: WriteableMemorySlice> WriteableMemorySlice for &mut T {
    fn write(&mut self, bytes: &[u8]) {
        T::write(self, bytes);
    }
}

struct ModuleWriteableMemory {
    memory: crate::Memory,
    realloc: Function,
}

impl ModuleWriteableMemory {
    #[allow(unused)]
    fn new(memory: crate::Memory, realloc: Function) -> Self {
        Self { memory, realloc }
    }

    fn malloc(&mut self, align: usize, size: usize) -> Result<usize, ()> {
        let zero: JsValue = 0.into();

        let args = Array::of4(&zero, &zero, &(align as u32).into(), &(size as u32).into());
        let result = self
            .realloc
            .apply(&JsValue::UNDEFINED, &args)
            .expect("call capi_realloc");

        // TODO: realloc might run out of memory, in that case, we should propagate error to the user

        Ok(result.as_f64().expect("realloc should return a number") as usize)
    }
}

struct ModuleWriteableMemorySlice {
    start_offset: usize,
    data_buffer: Vec<u8>,
}

impl ModuleWriteableMemorySlice {
    fn new(start_offset: usize, size: usize) -> Self {
        Self {
            start_offset,
            data_buffer: Vec::with_capacity(size as _),
        }
    }
}

impl WriteableMemory for ModuleWriteableMemory {
    type Slice = ModuleWriteableMemorySlice;

    fn allocate(&mut self, align: usize, size: usize) -> Self::Slice {
        let start_offset = self.malloc(align, size).expect("calling malloc");
        ModuleWriteableMemorySlice::new(start_offset, size)
    }

    fn flush(&mut self, slice: Self::Slice) -> usize {
        self.memory
            .write_impl(slice.start_offset, &slice.data_buffer)
            .expect("write bytes to buffer");

        slice.start_offset
    }
}

impl WriteableMemorySlice for ModuleWriteableMemorySlice {
    fn write(&mut self, bytes: &[u8]) {
        self.data_buffer.extend_from_slice(bytes);
    }
}
