use std::collections::HashMap;

use crate::{FuncId, Result};

use super::{Func, TypedFunc};

pub struct Exports {
    root: ExportsRoot,
}

impl Exports {
    pub(crate) fn new(root: ExportsRoot) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &ExportsRoot {
        &self.root
    }
}

pub struct ExportsRoot {
    exports: HashMap<String, FuncId>,
}

impl ExportsRoot {
    pub(crate) fn new(exports: HashMap<String, FuncId>) -> Self {
        Self { exports }
    }

    pub fn typed_func<Params, Return>(&self, name: &str) -> Result<TypedFunc<Params, Return>> {
        // TODO: proper name conversion
        let name = if name == "add-hello" {
            "addHello"
        } else {
            name
        };
        // TODO: convert unwrap to user error
        // panic!("Getting export: {}, HASH MAP: {:?}", name, self.exports);
        let func = Func::new(*self.exports.get(name).unwrap());
        Ok(TypedFunc::new(func))
    }
}
