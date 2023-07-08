use std::collections::HashMap;

use heck::ToLowerCamelCase;

use crate::Result;

use super::*;

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
    exports: HashMap<String, Func>,
}

impl ExportsRoot {
    pub(crate) fn new(exports: HashMap<String, Func>) -> Self {
        Self { exports }
    }

    pub fn typed_func<Params, Return>(&self, name: &str) -> Result<TypedFunc<Params, Return>> {
        // TODO: converting in the opposite direction when storing would be slightly faster
        let name = name.to_lower_camel_case();

        // TODO: convert unwrap to user error
        let func = self.exports.get(&name).unwrap().clone();
        Ok(TypedFunc::new(func))
    }
}
