use std::collections::HashMap;

use super::*;

#[derive(Default)]
pub struct ResourceTable {
    pub(crate) input_streams: ResourceTableInner<Box<dyn HostInputStream>>,
    pub(crate) output_streams: ResourceTableInner<Box<dyn HostOutputStream>>,
}

impl ResourceTable {
    pub fn new() -> Self {
        Self {
            input_streams: ResourceTableInner::new(),
            output_streams: ResourceTableInner::new(),
        }
    }
}

pub(crate) struct ResourceTableInner<T> {
    items: HashMap<u32, T>,
    next_index: u32,
}

impl<T> ResourceTableInner<T> {
    pub(crate) fn new() -> Self {
        Self {
            items: HashMap::new(),
            next_index: 0,
        }
    }

    pub(crate) fn insert(&mut self, item: T) -> u32 {
        let index = self.next_index;
        self.items.insert(index, item);
        self.next_index += 1;
        index
    }

    #[allow(unused)]
    pub(crate) fn get(&self, index: u32) -> Option<&T> {
        self.items.get(&index)
    }

    pub(crate) fn get_mut(&mut self, index: u32) -> Option<&mut T> {
        self.items.get_mut(&index)
    }
}

impl<T> Default for ResourceTableInner<T> {
    fn default() -> Self {
        Self::new()
    }
}
