use std::marker::PhantomData;

use wasm_bridge_macros::{LiftJs, LowerJs, SizeDescription};

pub struct Resource<T> {
    id: u32,
    _phantom: PhantomData<Box<T>>,
}

impl<T> Resource<T> {
    pub fn new_own(id: u32) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn rep(&self) -> u32 {
        self.id
    }
}

pub struct ResourceAny {
    id: u32,
}
