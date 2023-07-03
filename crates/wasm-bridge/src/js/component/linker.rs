use crate::component::*;
use std::marker::PhantomData;

pub struct Linker<T> {
    _phantom: PhantomData<T>,
}

impl<T> Linker<T> {
    pub fn instantiate(_store: impl AsContextMut<Data = T>, component: &Component) -> Instance {
        Instance
    }
}
