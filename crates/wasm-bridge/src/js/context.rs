use crate::Store;

pub trait AsContext {
    type Data;

    fn as_context(&self) -> &Store<Self::Data>;
}

impl<T> AsContext for Store<T> {
    type Data = T;

    fn as_context(&self) -> &Store<Self::Data> {
        self
    }
}

impl<'a, T: AsContext> AsContext for &'a T {
    type Data = T::Data;

    fn as_context(&self) -> &Store<Self::Data> {
        T::as_context(*self)
    }
}

impl<'a, T: AsContext> AsContext for &'a mut T {
    type Data = T::Data;

    fn as_context(&self) -> &Store<Self::Data> {
        T::as_context(*self)
    }
}

pub trait AsContextMut: AsContext {
    fn as_context_mut(&mut self) -> &mut Store<Self::Data>;
}

impl<T> AsContextMut for Store<T> {
    fn as_context_mut(&mut self) -> &mut Store<Self::Data> {
        self
    }
}

impl<'a, T: AsContextMut> AsContextMut for &'a mut T {
    fn as_context_mut(&mut self) -> &mut Store<Self::Data> {
        T::as_context_mut(*self)
    }
}
