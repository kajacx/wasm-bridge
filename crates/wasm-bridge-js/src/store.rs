use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Store<T> {
    engine: Engine,
    _data: T,
}

impl<T> Store<T> {
    pub fn new(engine: &Engine, data: T) -> Self {
        Self {
            engine: engine.clone(),
            _data: data,
        }
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }
}
