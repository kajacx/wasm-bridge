use js_sys::Array;
use wasm_bindgen::JsValue;

use super::*;

impl Lower for i32 {
    fn to_abi<M: WriteableMemory>(&self, _memory: &M, args: &mut Vec<JsValue>) {
        args.push((*self).into());
    }

    fn write_to<M: WriteableMemory>(&self, _memory: &M, memory_slice: &mut M::Slice) {
        memory_slice.write(&self.to_le_bytes())
    }
}

impl Lower for u32 {
    fn to_abi<M: WriteableMemory>(&self, _memory: &M, args: &mut Vec<JsValue>) {
        args.push((*self).into());
    }

    fn write_to<M: WriteableMemory>(&self, _memory: &M, memory_slice: &mut M::Slice) {
        memory_slice.write(&self.to_le_bytes())
    }
}

impl<T: Lower> Lower for &[T] {
    fn to_abi<M: WriteableMemory>(&self, memory: &M, args: &mut Vec<JsValue>) {
        let addr = write_vec_data(self, memory) as u32;
        let len = self.len() as u32;

        // First address, then element count
        args.push(addr.into());
        args.push(len.into());
    }

    fn write_to<M: WriteableMemory>(&self, memory: &M, memory_slice: &mut M::Slice) {
        let addr = write_vec_data(self, memory) as u32;
        let len = self.len() as u32;

        addr.write_to(memory, memory_slice);
        len.write_to(memory, memory_slice);
    }
}

impl<T: Lower> Lower for Vec<T> {
    fn to_abi<M: WriteableMemory>(&self, memory: &M, args: &mut Vec<JsValue>) {
        self.as_slice().to_abi(memory, args)
    }

    fn write_to<M: WriteableMemory>(&self, memory: &M, memory_slice: &mut M::Slice) {
        self.as_slice().write_to(memory, memory_slice)
    }
}

fn write_vec_data<T: Lower, M: WriteableMemory>(data: &[T], memory: &M) -> usize {
    // Allocate space for all the elements
    let mut slice = memory.allocate(T::alignment(), T::flat_byte_size() * data.len());

    // Then write the elements to the slice buffer
    for elem in data {
        // FIXME: fill gaps in memory?
        elem.write_to(memory, &mut slice);
    }

    // Then actually write the slice buffer to memory
    memory.flush(slice)
}

impl Lower for () {
    fn to_abi<M: WriteableMemory>(&self, _memory: &M, _args: &mut Vec<JsValue>) {
        //no-op
    }

    fn write_to<M: WriteableMemory>(&self, _memory: &M, _memory_slice: &mut M::Slice) {
        //no-op
    }
}

impl<T: Lower> Lower for (T,) {
    fn to_abi<M: WriteableMemory>(&self, memory: &M, args: &mut Vec<JsValue>) {
        self.0.to_abi(memory, args)
    }

    fn write_to<M: WriteableMemory>(&self, memory: &M, memory_slice: &mut M::Slice) {
        self.0.write_to(memory, memory_slice)
    }
}

impl<T: Lower, U: Lower> Lower for (T, U) {
    fn to_abi<M: WriteableMemory>(&self, memory: &M, args: &mut Vec<JsValue>) {
        self.0.to_abi(memory, args);
        self.1.to_abi(memory, args);
    }

    fn write_to<M: WriteableMemory>(&self, memory: &M, memory_slice: &mut M::Slice) {
        self.0.write_to(memory, memory_slice);
        // FIXME: possible "gap" between the two values
        self.1.write_to(memory, memory_slice);
    }
}
