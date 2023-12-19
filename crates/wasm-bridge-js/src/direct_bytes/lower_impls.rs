use super::*;

impl Lower for i32 {
    type ReturnAbi = i32;

    fn to_return_abi<M: WriteableMemory>(&self, _memory: M) -> i32 {
        *self
    }

    fn write_to<M: WriteableMemory>(&self, _memory: M, memory_slice: &mut M::Slice) {
        memory_slice.write(&self.to_le_bytes())
    }
}

impl Lower for u32 {
    type ReturnAbi = i32; // TODO: should this be u32 instead? Does it matter?

    fn to_return_abi<M: WriteableMemory>(&self, _memory: M) -> Self::ReturnAbi {
        *self as _
    }

    fn write_to<M: WriteableMemory>(&self, _memory: M, memory_slice: &mut M::Slice) {
        memory_slice.write(&self.to_le_bytes())
    }
}

impl<T: Lower> Lower for Vec<T> {
    type ReturnAbi = (u32, u32);

    fn to_return_abi<M: WriteableMemory>(&self, mut memory: M) -> Self::ReturnAbi {
        // Allocate space for length and all elements
        let mut slice = memory.allocate(T::alignment(), T::flat_byte_size() * self.len());

        // Write the number of elements first
        (self.len() as u32).write_to(&mut memory, &mut slice);

        // Then write all the elements to the slice buffer
        for elem in self {
            elem.write_to(&mut memory, &mut slice);
        }

        // Then actually write the slice buffer to memory
        let addr = memory.flush(slice);

        // Return the address with the element count
        (addr as u32, self.len() as u32)
    }

    fn write_to<M: WriteableMemory>(&self, mut memory: M, memory_slice: &mut M::Slice) {
        // TODO: It just so happens that this is the same, but it should be extracted to a separate method
        let (address, len) = self.to_return_abi(&mut memory);

        address.write_to(&mut memory, memory_slice);
        len.write_to(memory, memory_slice);
    }
}
