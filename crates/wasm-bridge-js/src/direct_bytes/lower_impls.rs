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

fn write_vec_data<T: Lower, M: WriteableMemory>(data: &[T], memory: &M) -> usize {
    // Allocate space for all the elements
    let mut slice = memory.allocate(T::alignment(), T::flat_byte_size() * data.len());

    // Then write the elements to the slice buffer
    for elem in data {
        elem.write_to(memory, &mut slice);
    }

    // Then actually write the slice buffer to memory
    memory.flush(slice)
}

impl LowerArgs for () {
    fn to_fn_args<M: WriteableMemory>(self, _memory: &M) -> Array {
        Array::new()
    }
}

impl<T: Lower> LowerArgs for (T,) {
    fn to_fn_args<M: WriteableMemory>(self, memory: &M) -> Array {
        let mut args = vec![];
        self.0.to_abi(memory, &mut args);
        args.into_iter().collect()
    }
}

impl<T: Lower, U: Lower> LowerArgs for (T, U) {
    fn to_fn_args<M: WriteableMemory>(self, memory: &M) -> Array {
        let mut args = vec![];
        self.0.to_abi(memory, &mut args);
        self.1.to_abi(memory, &mut args);
        args.into_iter().collect()
    }
}

impl<T: Lower, U: Lower, V: Lower> LowerArgs for (T, U, V) {
    fn to_fn_args<M: WriteableMemory>(self, memory: &M) -> Array {
        let mut args = vec![];
        self.0.to_abi(memory, &mut args);
        self.1.to_abi(memory, &mut args);
        self.2.to_abi(memory, &mut args);
        args.into_iter().collect()
    }
}
