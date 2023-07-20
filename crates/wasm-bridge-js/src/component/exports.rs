use std::{collections::HashMap, marker::PhantomData, rc::Rc};

use anyhow::Context;
use heck::ToLowerCamelCase;
use js_sys::{Function, Object, Reflect};
use wasm_bindgen::JsValue;

use crate::{helpers::map_js_error, DropHandler, Result};

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
    exported_fns: HashMap<String, Func>,
}

impl ExportsRoot {
    pub(crate) fn new(exports: JsValue, closures: &Rc<[DropHandler]>) -> Result<Self> {
        let names = Object::get_own_property_names(&exports.clone().into());
        let mut exported_fns = HashMap::<String, Func>::new();

        for i in 0..names.length() {
            let name =
                Reflect::get_u32(&names, i).map_err(map_js_error("Get name of exported fn"))?;

            let function: Function = Reflect::get(&exports, &name)
                .map_err(map_js_error("Get exported fn"))?
                .into();

            exported_fns.insert(
                name.as_string().context("Export name should be a string")?,
                Func::new(function, closures.clone()),
            );
        }

        Ok(Self { exported_fns })
    }

    pub fn typed_func<Params, Return>(&self, name: &str) -> Result<TypedFunc<Params, Return>> {
        // TODO: converting in the opposite direction when storing would be slightly faster
        let name = name.to_lower_camel_case();

        let func = self
            .exported_fns
            .get(&name)
            .with_context(|| format!("Exported function '{name}' not found"))?
            .clone();
        Ok(TypedFunc::new(func))
    }

    pub fn instance(name: &str) -> ExportInstance {
        ExportInstance::new()
    }
}

pub struct ExportInstance<'a, 'b> {
    _phantom: PhantomData<(&'a (), &'b ())>,
}

impl<'a, 'b> ExportInstance<'a, 'b> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
