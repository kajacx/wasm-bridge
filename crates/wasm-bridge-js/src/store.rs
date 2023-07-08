use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Store<T> {
    engine: Engine,
    data: DataHandle<T>,
}

impl<T> Store<T> {
    pub fn new(engine: &Engine, data: T) -> Self {
        Self {
            engine: engine.clone(),
            data: Arc::new(Mutex::new(data)),
        }
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    // FIXME: calling this twice will panic
    pub fn data(&self) -> impl Deref<Target = T> + '_ {
        self.data.try_lock().unwrap()
    }

    pub fn data_mut(&mut self) -> impl DerefMut<Target = T> + '_ {
        self.data.try_lock().unwrap()
    }

    pub(crate) fn data_handle(&self) -> &DataHandle<T> {
        &self.data
    }
}

pub(crate) type DataHandle<T> = Arc<Mutex<T>>;

pub type StoreContext<'a, T> = &'a Store<T>;

pub type StoreContextMut<'a, T> = &'a mut Store<T>;
