use std::marker::PhantomData;

pub struct Caller<T> {
    _phantom: PhantomData<T>,
}

impl<T> Caller<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for Caller<T> {
    fn default() -> Self {
        Self::new()
    }
}
