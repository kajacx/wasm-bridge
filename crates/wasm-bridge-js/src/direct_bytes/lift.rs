use crate::Result;
use js_sys::Array;
use wasm_bindgen::JsValue;

use super::SizeDescription;
use crate::FromJsValue;

pub trait Lift: SizeDescription + Sized {
    /// Converts a returned value from an exported function to Self.
    fn from_js_return<M: ReadableMemory>(value: &JsValue, memory: &M) -> Result<Self>;

    /// Converts arguments to an imported function to Self.
    fn from_js_args<M: ReadableMemory>(args: &mut JsArgsReader, memory: &M) -> Result<Self>;

    /// Read from a slice of memory.
    fn read_from<M: ReadableMemory>(slice: &[u8], memory: &M) -> Result<Self>;

    /// Reads data for Self from a pointer.
    fn from_js_ptr_return<M: ReadableMemory>(value: &JsValue, memory: &M) -> Result<Self> {
        let addr = u32::from_js_value(value)? as usize;
        if Self::BYTE_SIZE <= 16 {
            let mut data = [0u8; 16];
            memory.read_to_slice(addr, &mut data[..Self::BYTE_SIZE]);
            Self::read_from(&data[..Self::BYTE_SIZE], memory)
        } else {
            let data = memory.read_to_vec(addr, Self::BYTE_SIZE);
            Self::read_from(&data, memory)
        }
    }
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

pub struct JsArgsReader {
    args: Array,
    index: u32,
    length: u32,
}

impl JsArgsReader {
    pub(crate) fn new(args: Array) -> Self {
        let length = args.length();
        Self {
            args,
            index: 0,
            length,
        }
    }
}

impl Iterator for JsArgsReader {
    type Item = JsValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.length {
            let value = self.args.get(self.index);
            self.index += 1;
            Some(value)
        } else {
            Option::None
        }
    }
}
