use std::collections::HashMap;

use super::*;

pub(crate) struct ResourceTable<T> {
    items: HashMap<u32, T>,
    next_index: u32,
}

impl<T> ResourceTable<T> {
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

impl<T> Default for ResourceTable<T> {
    fn default() -> Self {
        Self::new()
    }
}
