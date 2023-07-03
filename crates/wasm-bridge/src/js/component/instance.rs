use std::marker::PhantomData;

pub struct Instance {}

pub struct InstancePre<T> {
    _phantom: PhantomData<T>,
}
