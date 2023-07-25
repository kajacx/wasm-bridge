use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
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
            data: Rc::new(RefCell::new(data)),
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

pub(crate) type DataHandle<T> = Rc<RefCell<T>>;

pub type StoreContext<'a, T> = &'a T;

pub type StoreContextMut<'a, T> = &'a mut T;

impl<T> Clone for Store<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            engine: self.engine.clone(),
        }
    }
}
