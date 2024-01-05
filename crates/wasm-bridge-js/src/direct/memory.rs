use std::{cell::RefCell, ops::Deref, rc::Rc};

use anyhow::Context;
use js_sys::{Array, Function};
use wasm_bindgen::JsValue;

use crate::{helpers::map_js_error, Result};

use super::{Lower, ReadableMemory, WriteableMemory};

#[derive(Debug, Clone)]
pub struct ModuleMemoryInner {
    memory: crate::Memory,
    realloc: Function,
}

#[derive(Debug, Clone, Default)]
pub struct ModuleMemory(Rc<RefCell<Option<ModuleMemoryInner>>>);

impl ModuleMemory {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Option::None)))
    }

    pub fn get(&self) -> impl Deref<Target = Option<ModuleMemoryInner>> + '_ {
        self.0.borrow()
    }

    pub fn set(&self, memory: crate::Memory, realloc: Function) {
        *self.0.borrow_mut() = Option::Some(ModuleMemoryInner::new(memory, realloc));
    }
}

impl ModuleMemoryInner {
    pub(crate) fn new(memory: crate::Memory, realloc: Function) -> Self {
        Self { memory, realloc }
    }

    fn malloc(&self, align: usize, size: usize) -> Result<usize> {
        let zero: JsValue = 0.into();

        // TODO: probably could re-use the same array
        let args = Array::of4(&zero, &zero, &(align as u32).into(), &(size as u32).into());
        let result = self
            .realloc
            .apply(&JsValue::UNDEFINED, &args)
            .map_err(map_js_error("call capi_realloc"))?;

        Ok(result.as_f64().context("realloc should return a number")? as usize)
    }
}

impl WriteableMemory for ModuleMemory {
    fn allocate(&self, align: usize, size: usize) -> Result<ByteBuffer> {
        let borrow = self.get();
        let address = borrow.as_ref().unwrap().malloc(align, size)?;
        Ok(ByteBuffer::new(address, size))
    }

    fn flush(&self, slice: ByteBuffer) -> usize {
        let borrow = self.get();
        borrow
            .as_ref()
            .unwrap()
            .memory
            .write_impl(slice.address, &slice.data)
            .expect("write bytes to buffer"); // TODO: Can this fail? If so, why?

        slice.address
    }
}

impl ReadableMemory for ModuleMemory {
    fn read_to_slice(&self, addr: usize, target: &mut [u8]) {
        let borrow = self.get();
        borrow
            .as_ref()
            .unwrap()
            .memory
            .read_impl(addr, target)
            .expect("Read memory should work")
    }
}

pub struct ByteBuffer {
    address: usize,
    data: Vec<u8>,
}

impl ByteBuffer {
    pub fn new(address: usize, size: usize) -> Self {
        Self {
            address,
            data: Vec::with_capacity(size as _),
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn skip(&mut self, num_bytes: usize) {
        // TODO: could just skip instead of writing 0s with some unsafe magic
        self.data.extend((0..num_bytes).map(|_| 0));
    }

    pub fn write<T: Lower, M: WriteableMemory>(&mut self, value: &T, memory: &M) -> Result<()> {
        value.write_to(self, memory)
    }
}
