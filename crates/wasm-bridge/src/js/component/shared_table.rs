use slab::Slab;
use std::{ops::Deref, rc::Rc};
use try_lock::{Locked, TryLock};

#[derive(Clone, Debug)]
pub struct SharedTable<T>(Rc<TryLock<Slab<T>>>);

impl<T> SharedTable<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&self, value: T) -> u32 {
        self.0.try_lock().unwrap().insert(value) as u32
    }

    pub fn get(&self, key: u32) -> Option<impl Deref<Target = T> + '_> {
        let lock = self.0.try_lock().unwrap();
        if lock.get(key as usize).is_some() {
            Some(ValueAccess(lock, key as usize))
        } else {
            None
        }
    }

    pub fn remove(&self, key: u32) -> Option<T> {
        self.0.try_lock().unwrap().try_remove(key as usize)
    }
}

struct ValueAccess<'a, T>(Locked<'a, Slab<T>>, usize);

impl<'a, T> Deref for ValueAccess<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.get(self.1).unwrap()
    }
}

impl<T> Default for SharedTable<T> {
    fn default() -> Self {
        Self(Rc::new(TryLock::new(Slab::default())))
    }
}

#[cfg(test)]
#[wasm_bindgen_test::wasm_bindgen_test]
fn test() {
    let table = SharedTable::<i32>::new();

    let key1 = table.insert(1);
    let key2 = table.insert(2);

    assert_eq!(*table.get(key1).unwrap(), 1);
    assert_eq!(*table.get(key2).unwrap(), 2);

    table.remove(key2).unwrap();
    assert!(table.get(key2).is_none());
    assert!(table.remove(key2).is_none());
}
