use std::{cell::RefCell, ops::Deref, rc::Rc};

use anyhow::{bail, Context};
use js_sys::{Array, Function};
use wasm_bindgen::JsValue;

use crate::{helpers::map_js_error, Result};

use super::{Lower, ReadableMemory, WriteableMemory};

#[derive(Debug, Clone)]
pub(crate) struct ModuleMemory {
    pub(crate) memory: crate::Memory,
    pub(crate) realloc: Function,
}

impl ModuleMemory {
    pub(crate) fn new(memory: crate::Memory, realloc: Function) -> Self {
        Self { memory, realloc }
    }

    fn malloc(&self, align: usize, size: usize) -> Result<usize> {
        thread_local! {
            static ARGS: [Array; 9] = [0,1,2,3,4,5,6,7,8].map(|align| {
                let array = Array::new_with_length(4);
                array.set(0, 0.into());
                array.set(1, 0.into());
                array.set(2, align.into());
                array
            });
        }

        if align > 8 {
            bail!("Align must be at most 8, it is {align} instead");
        }

        ARGS.with(|align_args| {
            let args = &align_args[align];
            args.set(3, (size as u32).into());

            let result = self
                .realloc
                .apply(&JsValue::UNDEFINED, args)
                .map_err(map_js_error("call capi_realloc"))?;

            Ok(result.as_f64().context("realloc should return a number")? as usize)
        })
    }
}

impl WriteableMemory for ModuleMemory {
    fn allocate(&self, align: usize, size: usize) -> Result<ByteBuffer> {
        let address = self.malloc(align, size)?;
        Ok(ByteBuffer::new(address, size))
    }

    fn flush(&self, slice: ByteBuffer) -> usize {
        self.memory
            .write_impl(slice.address, &slice.data)
            .expect("write bytes to buffer"); // TODO: Can this fail? If so, why?

        slice.address
    }
}

impl ReadableMemory for ModuleMemory {
    fn read_to_slice(&self, addr: usize, target: &mut [u8]) {
        self.memory
            .read_impl(addr, target)
            .expect("read bytes from memory")
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct LazyModuleMemory(Rc<RefCell<Option<ModuleMemory>>>);

impl LazyModuleMemory {
    pub(crate) fn new() -> Self {
        Self(Rc::new(RefCell::new(Option::None)))
    }

    pub(crate) fn get(&self) -> impl Deref<Target = Option<ModuleMemory>> + '_ {
        self.0.borrow()
    }

    pub(crate) fn set(&self, module_memory: ModuleMemory) {
        *self.0.borrow_mut() = Some(module_memory);
    }
}

impl WriteableMemory for LazyModuleMemory {
    fn allocate(&self, align: usize, size: usize) -> Result<ByteBuffer> {
        self.get()
            .as_ref()
            .expect("initialized lazy memory")
            .allocate(align, size)
    }

    fn flush(&self, buffer: ByteBuffer) -> usize {
        self.get()
            .as_ref()
            .expect("initialized lazy memory")
            .flush(buffer)
    }
}

impl ReadableMemory for LazyModuleMemory {
    fn read_to_slice(&self, addr: usize, target: &mut [u8]) {
        self.get()
            .as_ref()
            .expect("initialized lazy memory")
            .read_to_slice(addr, target)
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
        self.data.extend((0..num_bytes).map(|_| 0));
    }

    pub fn write<T: Lower, M: WriteableMemory>(&mut self, value: &T, memory: &M) -> Result<()> {
        value.write_to(self, memory)
    }
}
