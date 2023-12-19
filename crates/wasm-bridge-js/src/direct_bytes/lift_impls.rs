use anyhow::Result;

use super::*;

impl Lift for i32 {
    type Abi = i32;

    fn from_abi<M: ReadableMemory>(abi: Self::Abi, _memory: M) -> Result<Self> {
        Ok(abi)
    }

    fn read_from<M: ReadableMemory>(slice: &[u8], _memory: M) -> Result<Self> {
        Ok(i32::from_le_bytes(slice.try_into()?))
    }
}

impl Lift for u32 {
    type Abi = i32;

    fn from_abi<M: ReadableMemory>(abi: Self::Abi, _memory: M) -> Result<Self> {
        Ok(abi as _)
    }

    fn read_from<M: ReadableMemory>(slice: &[u8], _memory: M) -> Result<Self> {
        Ok(u32::from_le_bytes(slice.try_into()?))
    }
}

impl<T: Lift> Lift for Vec<T> {
    type Abi = u32;

    fn from_abi<M: ReadableMemory>(addr: Self::Abi, memory: M) -> Result<Self> {
        let mut addr_and_len = [0u8; 8];
        memory.read_to_slice(addr as usize, &mut addr_and_len);

        Self::read_from(&addr_and_len, memory)
    }

    fn read_from<M: ReadableMemory>(addr_and_len: &[u8], memory: M) -> Result<Self> {
        let addr = u32::from_le_bytes(addr_and_len[0..4].try_into().unwrap()) as usize;
        let len = u32::from_le_bytes(addr_and_len[4..8].try_into().unwrap()) as usize;

        let size = T::flat_byte_size();
        let data = memory.read_to_vec(addr, size * len);

        let mut result = Vec::with_capacity(len);
        for i in 0..len {
            result.push(T::read_from(&data[i * size..(i + 1) * size], &memory)?);
        }
        Ok(result)
    }
}

// TODO: fix this
impl<T: Lift> Lift for (T,) {
    type Abi = T::Abi;
    fn from_abi<M: ReadableMemory>(abi: Self::Abi, memory: M) -> Result<Self> {
        Ok((T::from_abi(abi, memory)?,))
    }
    fn read_from<M: ReadableMemory>(slice: &[u8], memory: M) -> Result<Self> {
        Ok((T::read_from(slice, memory)?,))
    }
}
