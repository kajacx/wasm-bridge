use anyhow::Result;

use super::*;

impl Lift for i32 {
    fn read_from<M: ReadableMemory>(memory: M, addr: usize) -> Result<Self> {
        let mut le_bytes = [0u8; 4];
        memory.read_slice(addr, &mut le_bytes);
        Ok(i32::from_le_bytes(le_bytes))
    }
}

impl Lift for u32 {
    fn read_from<M: ReadableMemory>(memory: M, addr: usize) -> Result<Self> {
        let mut le_bytes = [0u8; 4];
        memory.read_slice(addr, &mut le_bytes);
        Ok(u32::from_le_bytes(le_bytes))
    }
}

impl<T: Lift> Lift for Vec<T> {
    fn read_from<M: ReadableMemory>(memory: M, addr: usize) -> Result<Self> {
        
        let data = memory.read_to_vec(addr, T::flat_byte_size() * )
    }
}
