use std::{clone, collections::HashMap, marker::PhantomData, rc::Rc};

use anyhow::Context;
use heck::ToLowerCamelCase;
use js_sys::{Function, Object, Reflect, WebAssembly};
use wasm_bindgen::JsValue;

use crate::{
    direct_bytes::ModuleWriteableMemory,
    helpers::{self, map_js_error},
    DropHandle, Result,
};

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
    pub(crate) fn new(exports: JsValue) -> Result<Self> {
        let mut exported_js_fns = HashMap::<String, Function>::new();
        let mut post_return_js_fns = HashMap::<String, Function>::new();
        let mut memory = Option::<JsValue>::None;

        const POST_RETURN_PREFIX: &'static str = "cabi_post_";

        let names = Object::get_own_property_names(&exports.clone().into());
        for i in 0..names.length() {
            let name =
                Reflect::get_u32(&names, i).map_err(map_js_error("Get name of an export"))?;
            let name_string = name.as_string().context("Export name should be a string")?;

            let exported =
                Reflect::get(&exports, &name).map_err(map_js_error("Get exported value"))?;

            if exported.is_function() {
                if name_string.starts_with(POST_RETURN_PREFIX) {
                    post_return_js_fns.insert(name_string, exported.into());
                } else {
                    exported_js_fns.insert(name_string, exported.into());
                }
            } else if name_string == "memory" {
                memory = Some(exported);
            }
        }

        let memory: WebAssembly::Memory = memory.context("Did not find memory export")?.into();
        let memory = crate::Memory::new(memory);

        let realloc = exported_js_fns
            .get("cabi_realloc")
            .context("Cannot find cabi_realloc exported fn")?;

        let memory = ModuleWriteableMemory::new(memory, realloc.clone());

        let mut exported_fns = HashMap::<String, Func>::new();
        for (name, func) in exported_js_fns.into_iter() {
            let post_return_name = format!("{POST_RETURN_PREFIX}{name}");
            let post_return = post_return_js_fns.get(&post_return_name).map(Clone::clone);
            exported_fns.insert(name, Func::new(func, post_return, memory.clone()));
        }

        Ok(Self { exported_fns })
    }

    pub fn typed_func<Params, Return>(&self, name: &str) -> Result<TypedFunc<Params, Return>> {
        // TODO: converting in the opposite direction when storing would be slightly faster
        // let name = name.to_lower_camel_case();

        let func = self
            .exported_fns
            .get(name)
            .with_context(|| format!("Exported function '{name}' not found"))?
            .clone();

        Ok(TypedFunc::new(func))
    }

    pub fn instance(&self, name: &str) -> Option<ExportInstance> {
        todo!("named interface nesting")
        // Some(ExportInstance::new(
        //     self.exported_objects
        //         .get(name)
        //         // TODO: This is a workaround for https://github.com/bytecodealliance/jco/issues/110
        //         .or_else(|| self.exported_objects.get(&name.to_lower_camel_case()))?,
        // ))
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
