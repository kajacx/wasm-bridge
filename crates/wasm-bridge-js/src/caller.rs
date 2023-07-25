use std::ops::{Deref, DerefMut};

use crate::*;

#[derive(Debug)]
pub struct Caller<T> {
    store: Store<T>,
}

impl<T> Caller<T> {
    pub(crate) fn new(handle: DataHandle<T>) -> Self {
        Self {
            store: Store::from_handle(handle),
        }
    }

    pub fn data(&self) -> impl Deref<Target = T> + '_ {
        self.store.data()
    }

    pub fn data_mut(&mut self) -> impl DerefMut<Target = T> + '_ {
        self.store.data_mut()
    }
}

impl<T> Clone for Caller<T> {
    fn clone(&self) -> Self {
        Self {
            store: self.store.clone(),
        }
    }
}

impl<T> AsContext for Caller<T> {
    type Data = T;

    fn as_context(&self) -> &Store<Self::Data> {
        &self.store
    }
}

impl<T> AsContextMut for Caller<T> {
    fn as_context_mut(&mut self) -> &mut Store<Self::Data> {
        &mut self.store
    }
}
