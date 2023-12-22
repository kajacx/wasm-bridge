use super::*;
use crate::Result;
use wasm_bindgen::JsValue;

impl Lower for i32 {
    fn num_args() -> usize {
        1
    }

    fn to_abi<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, _memory: &M) {
        args.push((*self).into());
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, _memory: &M) -> Result<()> {
        buffer.write(&self.to_le_bytes());
        Ok(())
    }
}

impl Lower for u32 {
    fn num_args() -> usize {
        1
    }

    fn to_abi<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, _memory: &M) {
        args.push((*self).into());
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, _memory: &M) -> Result<()> {
        buffer.write(&self.to_le_bytes());
        Ok(())
    }
}

impl<T: Lower> Lower for &[T] {
    fn num_args() -> usize {
        2
    }

    fn to_abi<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) {
        let addr = write_vec_data(self, memory).expect("TODO: user error write vec data") as u32;
        let len = self.len() as u32;

        // First address, then element count
        args.push(addr.into());
        args.push(len.into());
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        let addr = write_vec_data(self, memory)? as u32;
        let len = self.len() as u32;

        addr.write_to(buffer, memory)?;
        len.write_to(buffer, memory)?;

        Ok(())
    }
}

impl<T: Lower> Lower for Vec<T> {
    fn num_args() -> usize {
        2
    }

    fn to_abi<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) {
        self.as_slice().to_abi(args, memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        self.as_slice().write_to(buffer, memory)
    }
}

fn write_vec_data<T: Lower, M: WriteableMemory>(data: &[T], memory: &M) -> Result<usize> {
    // Allocate space for all the elements
    let mut buffer = memory.allocate(T::alignment(), T::flat_byte_size() * data.len())?;

    // Then write the elements to the slice buffer
    for elem in data {
        elem.write_to(&mut buffer, memory)?;
    }

    // Then actually write the slice buffer to memory and return the address
    Ok(memory.flush(buffer))
}

impl Lower for () {
    fn num_args() -> usize {
        0 // TODO: verify this
    }

    fn to_abi<M: WriteableMemory>(&self, _args: &mut Vec<JsValue>, _memory: &M) {
        //no-op
    }

    fn write_to<M: WriteableMemory>(&self, _buffer: &mut ByteBuffer, _memory: &M) -> Result<()> {
        Ok(())
    }
}

impl<T: Lower> Lower for (T,) {
    fn num_args() -> usize {
        T::num_args()
    }

    fn to_abi<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) {
        self.0.to_abi(args, memory)
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        self.0.write_to(buffer, memory)
    }
}

impl<T: Lower, U: Lower> Lower for (T, U) {
    fn num_args() -> usize {
        T::num_args() + U::num_args()
    }

    fn to_abi<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) {
        self.0.to_abi(args, memory);
        self.1.to_abi(args, memory);
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        // CAREFUL!!!
        // `write_to` needs to fill the entire byte size of the pair,
        // or there would be unfilled "gaps" and the data would get shifted.
        let layout = Self::layout();

        self.0.write_to(buffer, memory)?;
        buffer.skip(layout[2] - layout[1]);

        self.1.write_to(buffer, memory)?;
        buffer.skip(layout[4] - layout[2]);

        Ok(())
    }
}

impl<T: Lower, U: Lower, V: Lower> Lower for (T, U, V) {
    fn num_args() -> usize {
        T::num_args() + U::num_args() + V::num_args()
    }

    fn to_abi<M: WriteableMemory>(&self, args: &mut Vec<JsValue>, memory: &M) {
        self.0.to_abi(args, memory);
        self.1.to_abi(args, memory);
        self.2.to_abi(args, memory);
    }

    fn write_to<M: WriteableMemory>(&self, buffer: &mut ByteBuffer, memory: &M) -> Result<()> {
        // CAREFUL!!!
        // `write_to` needs to fill the entire byte size of the tuple,
        // or there would be unfilled "gaps" and the data would get shifted.
        let layout = Self::layout();

        self.0.write_to(buffer, memory)?;
        buffer.skip(layout[2] - layout[1]);

        self.1.write_to(buffer, memory)?;
        buffer.skip(layout[4] - layout[2]);

        self.2.write_to(buffer, memory)?;
        buffer.skip(layout[6] - layout[4]);

        Ok(())
    }
}
