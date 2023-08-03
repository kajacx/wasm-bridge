use std::{collections::HashMap, marker::PhantomData, rc::Rc};

use anyhow::Context;
use heck::ToLowerCamelCase;
use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;

use crate::{helpers::map_js_error, DropHandle, Result};

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
    exported_objects: HashMap<String, ExportsRoot>, // TODO: not really great design
}

impl ExportsRoot {
    pub(crate) fn new(exports: JsValue, closures: &Rc<[DropHandle]>) -> Result<Self> {
        let names = Object::get_own_property_names(&exports.clone().into());
        let mut exported_fns = HashMap::<String, Func>::new();
        let mut exported_objects = HashMap::<String, ExportsRoot>::new();

        for i in 0..names.length() {
            let name =
                Reflect::get_u32(&names, i).map_err(map_js_error("Get name of an export"))?;
            let name_string = name.as_string().context("Export name should be a string")?;

            let exported =
                Reflect::get(&exports, &name).map_err(map_js_error("Get exported value"))?;

            if exported.is_function() {
                exported_fns.insert(name_string, Func::new(exported.into(), closures.clone()));
            } else if exported.is_object() {
                exported_objects.insert(name_string, ExportsRoot::new(exported, closures)?);
            } else {
                return Err(map_js_error(
                    "Exported value must be a function or an object",
                )(exported));
            }
        }

        Ok(Self {
            exported_fns,
            exported_objects,
        })
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

    pub fn instance(&self, name: &str) -> Option<ExportInstance> {
        Some(ExportInstance::new(
            self.exported_objects
                .get(name)
                // TODO: This is a workaround for https://github.com/bytecodealliance/jco/issues/110
                .or_else(|| self.exported_objects.get(&name.to_lower_camel_case()))?,
        ))
    }
}

pub struct ExportInstance<'a, 'b> {
    root: &'a ExportsRoot, // TODO: this is not the root, refactor
    _phantom: PhantomData<&'b ()>,
}

impl<'a, 'b> ExportInstance<'a, 'b> {
    pub(crate) fn new(root: &'a ExportsRoot) -> Self {
        Self {
            root,
            _phantom: PhantomData,
        }
    }

    pub fn typed_func<Params, Return>(&self, name: &str) -> Result<TypedFunc<Params, Return>> {
        self.root.typed_func(name)
    }
}
