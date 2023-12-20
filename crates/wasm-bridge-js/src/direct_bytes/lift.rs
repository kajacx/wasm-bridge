use anyhow::Result;
use wasm_bindgen::JsValue;

use super::SizeDescription;

pub trait Lift: SizeDescription + Sized {
    fn from_js_return<M: ReadableMemory>(val: &JsValue, memory: &M) -> Result<Self>;

    fn read_from<M: ReadableMemory>(slice: &[u8], memory: &M) -> Result<Self>;
}

pub trait ReadableMemory {
    fn read_to_slice(&self, addr: usize, target: &mut [u8]);

    fn read_to_vec(&self, addr: usize, len: usize) -> Vec<u8> {
        // TODO: could do uninit memory with unsafe code
        let mut vec: Vec<u8> = (0..len).map(|_| 0).collect();
        self.read_to_slice(addr, &mut vec);
        vec
    }
}

impl<M: ReadableMemory> ReadableMemory for &M {
    fn read_to_slice(&self, addr: usize, target: &mut [u8]) {
        M::read_to_slice(self, addr, target)
    }
}
