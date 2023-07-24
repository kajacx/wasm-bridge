use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::*;

#[derive(Debug, Default)]
pub struct Store<T> {
    pub(crate) engine: Engine,
    pub(crate) data: DataHandle<T>,
}

impl<T> Store<T> {
    pub fn new(engine: &Engine, data: T) -> Self {
        Self {
            engine: engine.clone(),
            data: Rc::new(RefCell::new(data)),
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

    pub fn into_data(self) -> Option<T> {
        Some(Rc::into_inner(self.data)?.into_inner())
    }
}

impl<T> Clone for Store<T> {
    fn clone(&self) -> Self {
        Store {
            engine: self.engine.clone(),
            data: self.data.clone(),
        }
    }
}

pub(crate) type DataHandle<T> = Rc<RefCell<T>>;

pub type StoreContext<'a, T> = &'a T;

pub type StoreContextMut<'a, T> = &'a mut T;
