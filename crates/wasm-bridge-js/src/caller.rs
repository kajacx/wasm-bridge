use crate::DataHandle;

#[derive(Debug)]
pub struct Caller<T> {
    handle: DataHandle<T>,
}

impl<T> Caller<T> {
    pub fn new(handle: DataHandle<T>) -> Self {
        Self { handle }
    }
}

impl<T> Clone for Caller<T> {
    fn clone(&self) -> Self {
        Caller::new(self.handle.clone())
    }
}
