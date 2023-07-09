use std::{marker::PhantomData, rc::Rc};

use super::*;
use crate::{AsContextMut, DropHandler, Result};

pub struct Instance {
    exports: Exports,
    _closures: Rc<Vec<DropHandler>>,
}

impl Instance {
    pub(crate) fn new(exports: ExportsRoot, closures: Rc<Vec<DropHandler>>) -> Self {
        Self {
            exports: Exports::new(exports),
            _closures: closures,
        }
    }

    pub fn exports(&self, _store: impl AsContextMut) -> &Exports {
        &self.exports
    }
}

pub struct InstancePre<T> {
    _phantom: PhantomData<T>,
}

impl<T> InstancePre<T> {
    pub fn instantiate(&self, _store: impl AsContextMut<Data = T>) -> Result<Instance> {
        todo!()
    }
}
