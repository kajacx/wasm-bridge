use crate::conversions::FromJsValue;
use anyhow::{Context, Result};
use wasm_bindgen::JsValue;

use super::*;

impl Lift for i32 {
    fn from_js_return<M: ReadableMemory>(value: &JsValue, _memory: M) -> Result<Self> {
        Self::from_js_value(value)
    }

    fn read_from<M: ReadableMemory>(slice: &[u8], _memory: M) -> Result<Self> {
        Ok(i32::from_le_bytes(slice.try_into()?))
    }
}

impl Lift for u32 {
    fn from_js_return<M: ReadableMemory>(value: &JsValue, _memory: M) -> Result<Self> {
        Self::from_js_value(value)
    }

    fn read_from<M: ReadableMemory>(slice: &[u8], _memory: M) -> Result<Self> {
        Ok(u32::from_le_bytes(slice.try_into()?))
    }
}

impl<T: Lift> Lift for Vec<T> {
    fn from_js_return<M: ReadableMemory>(value: &JsValue, memory: M) -> Result<Self> {
        let addr = u32::from_js_value(value)? as usize;

        let mut addr_and_len = [0u8; 8];
        memory.read_to_slice(addr, &mut addr_and_len);

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

impl Lift for () {
    fn from_js_return<M: ReadableMemory>(_val: &JsValue, _memory: M) -> Result<Self> {
        Ok(())
    }

    fn read_from<M: ReadableMemory>(_slice: &[u8], _memory: M) -> Result<Self> {
        Ok(())
    }
}

impl<T: Lift> Lift for (T,) {
    fn from_js_return<M: ReadableMemory>(val: &JsValue, memory: M) -> Result<Self> {
        Ok((T::from_js_return(val, memory)?,))
    }

    fn read_from<M: ReadableMemory>(slice: &[u8], memory: M) -> Result<Self> {
        Ok((T::read_from(slice, memory)?,))
    }
}

impl<T: Lift, U: Lift> Lift for (T, U) {
    fn from_js_return<M: ReadableMemory>(val: &JsValue, memory: M) -> Result<Self> {
        let addr = u32::from_js_value(val)? as usize;
        let len = Self::flat_byte_size();

        // TODO: could probably re-use a static byte slice here
        let data = memory.read_to_vec(addr, len);
        Self::read_from(&data, memory)
    }

    fn read_from<M: ReadableMemory>(slice: &[u8], memory: M) -> Result<Self> {
        let t = T::read_from(&slice[..T::flat_byte_size()], &memory)?;

        let u_start = next_multiple_of(T::flat_byte_size(), Self::alignment());
        let u_end = u_start + U::flat_byte_size();
        let u = U::read_from(&slice[u_start..u_end], memory)?;

        Ok((t, u))
    }
}
