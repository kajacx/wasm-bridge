use std::ops::{Deref, DerefMut};

use crate::Store;

#[derive(Debug)]
pub struct Caller<T> {
    pub(crate) store: Store<T>,
}

impl<T> Caller<T> {
    pub(crate) fn new(store: Store<T>) -> Self {
        Self { store }
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
        Caller::new(self.store.clone())
    }
}
