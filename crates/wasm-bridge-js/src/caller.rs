use std::ops::{Deref, DerefMut};

use crate::DataHandle;

#[derive(Debug)]
pub struct Caller<T> {
    handle: DataHandle<T>,
}

impl<T> Caller<T> {
    pub(crate) fn new(handle: DataHandle<T>) -> Self {
        Self { handle }
    }

    // FIXME: calling this twice will panic
    pub fn data(&self) -> impl Deref<Target = T> + '_ {
        self.handle.try_lock().unwrap()
    }

    pub fn data_mut(&mut self) -> impl DerefMut<Target = T> + '_ {
        self.handle.try_lock().unwrap()
    }
}

impl<T> Clone for Caller<T> {
    fn clone(&self) -> Self {
        Caller::new(self.handle.clone())
    }
}
