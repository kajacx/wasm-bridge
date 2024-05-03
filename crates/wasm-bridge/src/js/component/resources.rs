use std::{any::Any, marker::PhantomData};

use slab::Slab;

pub struct Resource<T> {
    id: u32,
    _phantom: PhantomData<Box<T>>,
}

impl<T> Resource<T> {
    pub fn new_own(id: u32) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn rep(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ResourceAny {
    pub(crate) id: u32,
}

pub struct ResourceType;

impl ResourceType {
    pub fn host<T>() {}
}

// TODO: unify with wasi's resource table?
#[derive(Default)]
pub struct ResourceTable(Slab<Box<dyn Any>>);

impl ResourceTable {
    pub fn push<R: Any>(&mut self, value: R) -> Result<Resource<R>, ResourceTableError> {
        let index = self.0.insert(Box::new(value));
        Ok(Resource::new_own(index as u32))
    }

    pub fn get<R: Any>(&self, resource: &Resource<R>) -> Result<&R, ResourceTableError> {
        self.0
            .get(resource.rep() as usize)
            .ok_or(ResourceTableError::NotPresent)?
            .downcast_ref()
            .ok_or(ResourceTableError::WrongType)
    }

    pub fn get_mut<R: Any>(
        &mut self,
        resource: &Resource<R>,
    ) -> Result<&mut R, ResourceTableError> {
        self.0
            .get_mut(resource.rep() as usize)
            .ok_or(ResourceTableError::NotPresent)?
            .downcast_mut()
            .ok_or(ResourceTableError::WrongType)
    }

    pub fn delete<R: Any>(&mut self, resource: Resource<R>) -> Result<R, ResourceTableError> {
        *(self
            .0
            .try_remove(resource.rep() as usize)
            .ok_or(ResourceTableError::NotPresent)?
            .downcast()
            .map_err(|_| ResourceTableError::WrongType)?)
    }
}

#[derive(Debug)]
/// Errors returned by operations on `ResourceTable`
pub enum ResourceTableError {
    /// ResourceTable has no free keys
    Full,
    /// Resource not present in table
    NotPresent,
    /// Resource present in table, but with a different type
    WrongType,
    /// Resource cannot be deleted because child resources exist in the table. Consult wit docs for
    /// the particular resource to see which methods may return child resources.
    HasChildren,
}
