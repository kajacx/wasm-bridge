use std::collections::HashMap;

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
        // TODO: translate it in the opposite direction for better caching?
        let name = Self::translate_func_name(name);

        // TODO: convert unwrap to user error
        let func = self.exports.get(&name).unwrap().clone();
        Ok(TypedFunc::new(func))
    }

    fn translate_func_name(name: &str) -> String {
        use std::fmt::Write;

        let mut parts = name.split('-');
        let mut result = parts.next().expect("non-empty name").to_string();

        for next in parts {
            let first = &next[0..1];
            let rest = &next[1..];
            write!(result, "{}{}", first.to_uppercase(), rest).unwrap();
        }

        result
    }
}
