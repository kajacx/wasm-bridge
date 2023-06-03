use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Store<T> {
    engine: Engine,
    _data: T,
}

impl<T> Store<T> {
    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}
