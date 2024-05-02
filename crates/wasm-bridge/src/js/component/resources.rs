use std::{any::Any, collections::HashMap, marker::PhantomData};

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
pub struct ResourceTable {
    items: HashMap<u32, Box<dyn Any>>,
    next_index: u32,
}

impl ResourceTable {
    pub fn push<R: Any>(&mut self, value: R) -> Result<Resource<R>, ResourceTableError> {
        let index = self.next_index;
        self.next_index += 1;

        self.items.insert(index, Box::new(value));
        Ok(Resource::new_own(index))
    }

    pub fn get<R: Any>(&self, resource: &Resource<R>) -> Result<&R, ResourceTableError> {
        self.items
            .get(&resource.rep())
            .ok_or(ResourceTableError::NotPresent)?
            .downcast_ref()
            .ok_or(ResourceTableError::WrongType)
    }

    pub fn delete<R: Any>(&mut self, resource: Resource<R>) -> Result<R, ResourceTableError> {
        *(self
            .items
            .remove(&resource.rep())
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
