use crate::Store;

pub trait AsContext {
    type Data;
}

impl<T> AsContext for Store<T> {
    type Data = T;
}

impl<'a, T: AsContext<Data = T>> AsContext for &'a T {
    type Data = T;
}

impl<'a, T: AsContext<Data = T>> AsContext for &'a mut T {
    type Data = T;
}

pub trait AsContextMut: AsContext {
    fn as_context_mut(&mut self) -> &mut Store<Self::Data>;
}

impl<T> AsContextMut for Store<T> {
    fn as_context_mut(&mut self) -> &mut Store<Self::Data> {
        self
    }
}

impl<'a, T: AsContextMut<Data = T>> AsContextMut for &'a mut T {
    fn as_context_mut(&mut self) -> &mut Store<Self::Data> {
        T::as_context_mut(*self)
    }
}
