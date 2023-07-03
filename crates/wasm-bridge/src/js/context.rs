use crate::Store;

pub trait AsContext {
    type Data;
}

impl<T> AsContext for Store<T> {
    type Data = T;
}

impl<'a, T: AsContext> AsContext for &'a T {
    type Data = T;
}

impl<'a, T: AsContext> AsContext for &'a mut T {
    type Data = T;
}

pub trait AsContextMut: AsContext {}

impl<T> AsContextMut for Store<T> {}

impl<'a, T: AsContextMut> AsContextMut for &'a mut T {}
