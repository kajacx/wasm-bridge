use atomic_refcell::AtomicRefCell;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::*;

#[derive(Debug, Default)]
pub struct Store<T> {
    engine: Engine,
    data: DataHandle<T>,
}

impl<T> Store<T> {
    pub fn new(engine: &Engine, data: T) -> Self {
        Self {
            engine: engine.clone(),
            data: Arc::new(AtomicRefCell::new(data)),
        }
    }

    pub(crate) fn from_handle(handle: DataHandle<T>) -> Self {
        Self {
            engine: Engine::default(), // Engine is unused, so this is file for now
            data: handle,
        }
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn data(&self) -> impl Deref<Target = T> + '_ {
        self.data.borrow()
    }

    pub fn data_mut(&mut self) -> impl DerefMut<Target = T> + '_ {
        self.data.borrow_mut()
    }

    pub(crate) fn data_handle(&self) -> &DataHandle<T> {
        &self.data
    }
}

pub(crate) type DataHandle<T> = Arc<AtomicRefCell<T>>;

pub struct StoreContext<'a, T>(&'a T);

impl<'a, T> StoreContext<'a, T> {
    pub fn new(reference: &'a T) -> Self {
        Self(reference)
    }

    pub fn data(&self) -> &T {
        self.0
    }
}

pub struct StoreContextMut<'a, T>(&'a mut T);

impl<'a, T> StoreContextMut<'a, T> {
    pub fn new(reference: &'a mut T) -> Self {
        Self(reference)
    }

    pub fn data(&self) -> &T {
        self.0
    }

    pub fn data_mut(&mut self) -> &mut T {
        self.0
    }
}

impl<T> Clone for Store<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            engine: self.engine.clone(),
        }
    }
}
