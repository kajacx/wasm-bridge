use std::{collections::BTreeMap, io::Write};

use anyhow::{bail, Context};
use js_sys::Function;
use wasmtime_environ::{
    component::{
        CanonicalOptions, ComponentTranslation, ComponentTypes, ComponentTypesBuilder, Export,
        GlobalInitializer, InstantiateModule, LoweredIndex, Trampoline, TrampolineIndex,
        Translator, TypeFuncIndex,
    },
    wasmparser::{Validator, WasmFeatures},
    ModuleTranslation, PrimaryMap, ScopeVec, StaticModuleIndex, Tunables,
};
use wit_component::DecodedWasm;
use wit_parser::{Resolve, SizeAlign, World, WorldId, WorldKey};

use crate::{helpers::map_js_error, Result};

pub(crate) struct ComponentFiles {
    pub instantiate: Function,
    pub wasm_cores: Vec<(String, Vec<u8>)>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ComponentLoader {}

impl ComponentLoader {
    pub fn generate_files(bytes: &[u8]) -> Result<ComponentFiles> {
        let transpiled = transpile(bytes)?;
        let files = transpiled.files;

        let mut wasm_cores = Vec::<(String, Vec<u8>)>::new();
        let mut instantiate = Option::<Function>::None;

        for (name, bytes) in files.into_iter() {
            if name.ends_with(".wasm") {
                wasm_cores.push((name, bytes));
            } else if name.ends_with(".js") {
                tracing::debug!("js loader\n\n{}", String::from_utf8_lossy(&bytes));
                // panic!("{}", String::from_utf8_lossy(&bytes));
                // TODO: test that instantiate is not already Some?
                instantiate = Some(load_instantiate_fn(&bytes)?);
            }
        }

        let instantiate = instantiate.context("Missing component.js file")?;

        Ok(ComponentFiles {
            instantiate,
            wasm_cores,
        })
    }
}

fn load_instantiate_fn(bytes: &[u8]) -> Result<Function> {
    let text = std::str::from_utf8(bytes)?;
    let text = modify_js(text);

    let result = js_sys::eval(&text).map_err(map_js_error("eval modified component.js file"))?;
    if !result.is_function() {
        bail!("instantiate should be a function");
    }

    Ok(result.into())
}

fn modify_js(text: &str) -> String {
    // function signature
    let text = text.replace("export async function", "function");

    // remove all awaits
    let text = text.replace("await ", "");

    // remove Promise.all call - not necessary
    // let regex = Regex::new(".*Promise\\.all.*").unwrap();
    // let text = regex.replace_all(&text, "");

    // Final update
    let text = format!("(() => {{\n{text}\nreturn instantiate;\n}})()\n");

    text
}

pub fn transpile(component: &[u8]) -> anyhow::Result<()> {
    let decoded =
        wit_component::decode(component).context("Failed to extract interface from component")?;

    let DecodedWasm::Component(resolve, world_id) = decoded else {
        anyhow::bail!("Attempt to parse wit package");
    };

    let scope = ScopeVec::new();
    let tunables = Tunables::default();
    let mut types = ComponentTypesBuilder::default();
    let mut validator = Validator::new_with_features(WasmFeatures {
        component_model: true,
        ..WasmFeatures::default()
    });

    let (component, modules) = Translator::new(&tunables, &mut validator, &mut types, &scope)
        .translate(component)
        .context("failed to parse the input component")?;

    // let modules: BTreeMap<StaticModuleIndex, core::Translation<'_>> = modules
    //     .into_iter()
    //     .map(|(_i, module)| core::Translation::new(module))
    //     .collect::<Result<_>>()?;

    let types = types.finish();
    let mut files = BTreeMap::new();

    // Insert all core wasm modules into the generated `Files` which will
    // end up getting used in the `generate_instantiate` method.
    for (i, module) in modules.iter() {
        files.insert(core_file_name(&"", i.as_u32()), module.wasm.to_vec());
    }

    let (imports, exports) = transpile_bindgen(
        &"", &component, &modules, &types, &resolve, world_id, &mut files,
    );
    todo!()
}

pub fn transpile_bindgen(
    name: &str,
    component: &ComponentTranslation,
    modules: &PrimaryMap<StaticModuleIndex, ModuleTranslation<'_>>,
    types: &ComponentTypes,
    resolve: &Resolve,
    id: WorldId,
    files: &mut BTreeMap<String, Vec<u8>>,
) -> (Vec<String>, Vec<(String, Export)>) {
    let mut bindgen = ModuleBindgen::default();
    // let mut bindgen = JsBindgen {
    //     local_names: LocalNames::default(),
    //     src: Source::default(),
    //     esm_bindgen: EsmBindgen::default(),
    //     core_module_cnt: 0,
    //     opts: &opts,
    //     all_intrinsics: BTreeSet::new(),
    // };
    // bindgen
    //     .local_names
    //     .exclude_intrinsics(Intrinsic::get_all_names());
    // bindgen.core_module_cnt = modules.len();

    // bindings is the actual `instantiate` method itself, created by this
    // structure.

    // populate reverse map from import names to world items
    let mut imports = BTreeMap::new();
    let mut exports = BTreeMap::new();
    for (key, _) in &resolve.worlds[id].imports {
        let name = resolve.name_world_key(key);
        imports.insert(name, key.clone());
    }
    for (key, _) in &resolve.worlds[id].exports {
        let name = resolve.name_world_key(key);
        exports.insert(name, key.clone());
    }

    let mut instantiator = Instantiator {
        // src: Source::default(),
        sizes: SizeAlign::default(),
        modules,
        // instances: Default::default(),
        gen: &mut bindgen,
        resolve,
        world: id,
        translation: component,
        component: &component.component,
        types,
        imports: &imports,
        exports: &exports,
        // lowering_options: Default::default(),
        // resource_tables_initialized: (0..component.component.num_resource_tables)
        //     .map(|_| false)
        //     .collect(),
    };

    instantiator.sizes.fill(resolve);
    instantiator.instantiate();
    // instantiator.gen.src.js(&instantiator.src.js);
    // instantiator.gen.src.js_init(&instantiator.src.js_init);

    instantiator.gen.finish_component(name, files);

    let exports = instantiator
        .gen
        .exports()
        .iter()
        .map(|(export_name, canon_export_name)| {
            let export = if canon_export_name.contains(':') {
                &instantiator.component.exports[*canon_export_name]
            } else {
                &instantiator.component.exports[*canon_export_name]
            };
            (export_name.to_string(), export.clone())
        })
        .collect();

    (bindgen.import_specifiers(), exports)
}

fn core_file_name(name: &str, idx: u32) -> String {
    let i_str = if idx == 0 {
        String::from("")
    } else {
        (idx + 1).to_string()
    };
    format!("{}.core{i_str}.wasm", name)
}

type LocalName = String;

enum Binding {
    Interface(BTreeMap<String, Binding>),
    Local(LocalName),
}

/// Maps a components exports and imports
#[derive(Default)]
pub struct ModuleBindgen {
    imports: BTreeMap<String, Binding>,
    exports: BTreeMap<String, Binding>,
    export_aliases: BTreeMap<String, String>,
}

impl ModuleBindgen {
    /// add imported function binding, using a path slice starting with the import specifier as its
    /// first segment
    /// arbitrary nesting of interfaces is supported in order to support virtual WASI interfaces
    /// only two-level nesting supports serialization into imports currently
    pub fn add_import(&mut self, path: &[String], func_name: String) {
        let mut iface = &mut self.imports;
        for i in 0..path.len() - 1 {
            if !iface.contains_key(&path[i]) {
                iface.insert(path[i].to_string(), Binding::Interface(BTreeMap::new()));
            }
            iface = match iface.get_mut(&path[i]).unwrap() {
                Binding::Interface(iface) => iface,
                Binding::Local(_) => panic!(
                    "Imported interface {} cannot be both a function and an interface",
                    &path[0..i].join(".")
                ),
            };
        }
        iface.insert(path[path.len() - 1].to_string(), Binding::Local(func_name));
    }

    /// add an exported function binding, optionally on an interface id or kebab name
    pub fn add_export_binding(
        &mut self,
        iface_id_or_kebab: Option<&str>,
        local_name: String,
        func_name: String,
    ) {
        let mut iface = &mut self.exports;
        if let Some(iface_id_or_kebab) = iface_id_or_kebab {
            // convert kebab names to camel case, leave ids as-is
            // let iface_id_or_kebab = if iface_id_or_kebab.contains(':') {
            //     iface_id_or_kebab.to_string()
            // } else {
            //     iface_id_or_kebab.to_lower_camel_case()
            // };
            if !iface.contains_key(iface_id_or_kebab) {
                iface.insert(
                    iface_id_or_kebab.to_string(),
                    Binding::Interface(BTreeMap::new()),
                );
            }
            iface = match iface.get_mut(iface_id_or_kebab).unwrap() {
                Binding::Interface(iface) => iface,
                Binding::Local(_) => panic!(
                    "Exported interface {} cannot be both a function and an interface",
                    iface_id_or_kebab
                ),
            };
        }
        iface.insert(func_name, Binding::Local(local_name));
    }

    /// get the exports (including exported aliases) from the bindgen
    pub fn exports<'a>(&'a self) -> Vec<(&'a str, &'a str)> {
        self.export_aliases
            .iter()
            .map(|(alias, name)| (alias.as_ref(), name.as_ref()))
            .chain(
                self.exports
                    .iter()
                    .map(|(name, _)| (name.as_ref(), name.as_ref())),
            )
            .collect()
    }

    /// get the final top-level import specifier list
    pub fn import_specifiers(&self) -> Vec<String> {
        self.imports.keys().map(|impt| impt.to_string()).collect()
    }
}

pub struct Instantiator<'a> {
    modules: &'a PrimaryMap<StaticModuleIndex, ModuleTranslation<'a>>,
    world: WorldId,

    sizes: SizeAlign,
    translation: &'a ComponentTranslation,
    component: &'a wasmtime_environ::component::Component,
    types: &'a ComponentTypes,

    exports: &'a BTreeMap<String, WorldKey>,
    imports: &'a BTreeMap<String, WorldKey>,
    resolve: &'a Resolve,
    gen: &'a mut ModuleBindgen,

    lowering_options:
        PrimaryMap<LoweredIndex, (&'a CanonicalOptions, TrampolineIndex, TypeFuncIndex)>,
}

impl<'a> Instantiator<'a> {
    fn instantiate(&mut self) {
        for i in 0..self.component.num_runtime_component_instances {
            // writeln!(self.src.js_init, "const instanceFlags{i} = new WebAssembly.Global({{ value: \"i32\", mutable: true }}, {});", wasmtime_environ::component::FLAG_MAY_LEAVE | wasmtime_environ::component::FLAG_MAY_ENTER);
        }

        for (i, trampoline) in self.translation.trampolines.iter() {
            let Trampoline::LowerImport {
                index,
                lower_ty,
                options,
            } = trampoline
            else {
                continue;
            };
            let i = self.lowering_options.push((options, i, *lower_ty));
            assert_eq!(i, *index);
        }

        // To avoid uncaught promise rejection errors, we attach an intermediate
        // Promise.all with a rejection handler, if there are multiple promises.
        if self.modules.len() > 1 {
            self.src.js_init.push_str("Promise.all([");
            for i in 0..self.modules.len() {
                if i > 0 {
                    self.src.js_init.push_str(", ");
                }
                self.src.js_init.push_str(&format!("module{}", i));
            }
            uwriteln!(self.src.js_init, "]).catch(() => {{}});");
        }

        for init in self.component.initializers.iter() {
            self.instantiation_global_initializer(init);
        }

        // Trampolines after initializers so we have static module indices
        for (i, trampoline) in self.translation.trampolines.iter() {
            self.trampoline(i, trampoline);
        }

        if self.gen.opts.instantiation {
            let js_init = mem::take(&mut self.src.js_init);
            self.src.js.push_str(&js_init);
        }

        self.exports(&self.component.exports);
    }

    fn instantiation_global_initializer(&mut self, init: &GlobalInitializer) {
        match init {
            GlobalInitializer::InstantiateModule(m) => match m {
                InstantiateModule::Static(idx, args) => self.instantiate_static_module(*idx, args),
                // This is only needed when instantiating an imported core wasm
                // module which while easy to implement here is not possible to
                // test at this time so it's left unimplemented.
                InstantiateModule::Import(..) => unimplemented!(),
            },
            GlobalInitializer::LowerImport { index, import } => {
                self.lower_import(*index, *import);
            }
            GlobalInitializer::ExtractMemory(m) => {
                let def = self.core_export(&m.export);
                let idx = m.index.as_u32();
                uwriteln!(self.src.js, "let memory{idx};");
                uwriteln!(self.src.js_init, "memory{idx} = {def};");
            }
            GlobalInitializer::ExtractRealloc(r) => {
                let def = self.core_def(&r.def);
                let idx = r.index.as_u32();
                uwriteln!(self.src.js, "let realloc{idx};");
                uwriteln!(self.src.js_init, "realloc{idx} = {def};",);
            }
            GlobalInitializer::ExtractPostReturn(p) => {
                let def = self.core_def(&p.def);
                let idx = p.index.as_u32();
                uwriteln!(self.src.js, "let postReturn{idx};");
                uwriteln!(self.src.js_init, "postReturn{idx} = {def};");
            }
            GlobalInitializer::Resource(_) => {}
        }
    }

    fn instantiate_static_module(&mut self, idx: StaticModuleIndex, args: &[CoreDef]) {
        // Build a JS "import object" which represents `args`. The `args` is a
        // flat representation which needs to be zip'd with the list of names to
        // correspond to the JS wasm embedding API. This is one of the major
        // differences between Wasmtime's and JS's embedding API.
        let mut import_obj = BTreeMap::new();
        for (module, name, arg) in self.modules[idx].imports(args) {
            let def = self.augmented_import_def(arg);
            let dst = import_obj.entry(module).or_insert(BTreeMap::new());
            let prev = dst.insert(name, def);
            assert!(
                prev.is_none(),
                "unsupported duplicate import of `{module}::{name}`"
            );
            assert!(prev.is_none());
        }
        let mut imports = String::new();
        if !import_obj.is_empty() {
            imports.push_str(", {\n");
            for (module, names) in import_obj {
                imports.push_str(&maybe_quote_id(module));
                imports.push_str(": {\n");
                for (name, val) in names {
                    imports.push_str(&maybe_quote_id(name));
                    uwriteln!(imports, ": {val},");
                }
                imports.push_str("},\n");
            }
            imports.push_str("}");
        }

        let i = self.instances.push(idx);
        let iu32 = i.as_u32();
        let instantiate = self.gen.intrinsic(Intrinsic::InstantiateCore);
        uwriteln!(self.src.js, "let exports{iu32};");
        uwriteln!(
            self.src.js_init,
            "({{ exports: exports{iu32} }} = await {instantiate}(await module{}{imports}));",
            idx.as_u32()
        );
    }

    fn lower_import(&mut self, index: LoweredIndex, import: RuntimeImportIndex) {
        let (options, trampoline, ty_func_idx) = self.lowering_options[index];

        let (import_index, path) = &self.component.imports[import];
        let (import_name, _) = &self.component.import_types[*import_index];
        let world_key = &self.imports[import_name];

        // nested interfaces only currently possible through mapping
        let (import_specifier, mut maybe_iface_member) =
            map_import(&self.gen.opts.map, &import_name);

        let (func, func_name, iface_name) =
            match &self.resolve.worlds[self.world].imports[world_key] {
                WorldItem::Function(func) => {
                    assert_eq!(path.len(), 0);
                    (func, import_name, None)
                }
                WorldItem::Interface(i) => {
                    assert_eq!(path.len(), 1);
                    let iface = &self.resolve.interfaces[*i];
                    let func = &iface.functions[&path[0]];
                    (
                        func,
                        &path[0],
                        Some(iface.name.as_deref().unwrap_or_else(|| import_name)),
                    )
                }
                WorldItem::Type(_) => unreachable!(),
            };

        let callee_name = match func.kind {
            FunctionKind::Freestanding => {
                let callee_name = self
                    .gen
                    .local_names
                    .get_or_create(
                        &format!("import:{}-{}", import_name, &func.name),
                        &func.name,
                    )
                    .0
                    .to_string();
                callee_name
            }
            FunctionKind::Method(ty) => {
                let ty = &self.resolve.types[ty];
                format!(
                    "{}.prototype.{}.call",
                    ty.name.as_ref().unwrap().to_upper_camel_case(),
                    func.item_name().to_lower_camel_case()
                )
            }
            FunctionKind::Static(ty) => {
                let ty = &self.resolve.types[ty];
                format!(
                    "{}.{}",
                    ty.name.as_ref().unwrap().to_upper_camel_case(),
                    func.item_name().to_lower_camel_case()
                )
            }
            FunctionKind::Constructor(ty) => {
                let ty = &self.resolve.types[ty];
                format!("new {}", ty.name.as_ref().unwrap().to_upper_camel_case())
            }
        };

        let nparams = self
            .resolve
            .wasm_signature(AbiVariant::GuestImport, func)
            .params
            .len();

        let resource_map = self.create_resource_map(func, ty_func_idx);

        uwrite!(self.src.js, "\nfunction trampoline{}", trampoline.as_u32());
        self.bindgen(
            nparams,
            false,
            if import_name.is_empty() {
                None
            } else {
                Some(import_name)
            },
            &callee_name,
            options,
            func,
            AbiVariant::GuestImport,
            resource_map,
        );
        uwriteln!(self.src.js, "");

        let (import_name, binding_name) = match func.kind {
            FunctionKind::Freestanding => (func_name.to_lower_camel_case(), callee_name),
            FunctionKind::Method(ty) | FunctionKind::Static(ty) | FunctionKind::Constructor(ty) => {
                let ty = &self.resolve.types[ty];
                (
                    ty.name.as_ref().unwrap().to_upper_camel_case(),
                    ty.name.as_ref().unwrap().to_upper_camel_case(),
                )
            }
        };

        // add the function import to the ESM bindgen
        if let Some(_iface_name) = iface_name {
            // mapping can be used to construct virtual nested namespaces
            // which is used eg to support WASI interface groupings
            if let Some(iface_member) = maybe_iface_member.take() {
                self.gen.esm_bindgen.add_import_binding(
                    &[
                        import_specifier,
                        iface_member.to_lower_camel_case(),
                        import_name,
                    ],
                    binding_name,
                );
            } else {
                self.gen
                    .esm_bindgen
                    .add_import_binding(&[import_specifier, import_name], binding_name);
            }
        } else {
            self.gen
                .esm_bindgen
                .add_import_binding(&[import_specifier], binding_name);
        }
    }
}
