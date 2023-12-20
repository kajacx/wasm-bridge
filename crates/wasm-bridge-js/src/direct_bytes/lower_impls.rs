use wasm_bindgen::JsValue;

use super::*;

impl Lower for i32 {
    fn to_abi<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, _memory: &M) {
        args.push((*self).into());
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, _memory: &M) {
        buffer.write(&self.to_le_bytes())
    }
}

impl Lower for u32 {
    fn to_abi<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, _memory: &M) {
        args.push((*self).into());
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, _memory: &M) {
        buffer.write(&self.to_le_bytes())
    }
}

impl<T: Lower> Lower for &[T] {
    fn to_abi<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) {
        let addr = write_vec_data(self, memory) as u32;
        let len = self.len() as u32;

        // First address, then element count
        args.push(addr.into());
        args.push(len.into());
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) {
        let addr = write_vec_data(self, memory) as u32;
        let len = self.len() as u32;

        addr.write_to(buffer, memory);
        len.write_to(buffer, memory);
    }
}

impl<T: Lower> Lower for Vec<T> {
    fn to_abi<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) {
        self.as_slice().to_abi(args, memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) {
        self.as_slice().write_to(buffer, memory)
    }
}

fn write_vec_data<T: Lower, M: WriteableMemory>(data: &[T], memory: &M) -> usize {
    // Allocate space for all the elements
    let mut slice = memory.allocate(T::alignment(), T::flat_byte_size() * data.len());

    // Then write the elements to the slice buffer
    for elem in data {
        elem.write_to(memory, &mut slice);
    }

    // Then actually write the slice buffer to memory and return the address
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
        // CAREFUL!!!
        // `write_to` needs to fill the entire byte size of the pair,
        // or there would be unfilled "gaps" and the data would get shifted.

        let align = Self::alignment();
        self.0.write_to_aligned(memory, memory_slice, align);
        self.1.write_to_aligned(memory, memory_slice, align);
    }
}
