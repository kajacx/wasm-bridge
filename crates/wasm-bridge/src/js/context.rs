use crate::Store;

pub trait AsContext {
    type Data;
}

impl<T> AsContext for Store<T> {
    type Data = T;
}

impl<'a, T: AsContext> AsContext for &'a T {
    type Data = T::Data;
}

impl<'a, T: AsContext> AsContext for &'a mut T {
    type Data = T::Data;
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
