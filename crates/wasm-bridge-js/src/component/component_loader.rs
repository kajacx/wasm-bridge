use std::io::Write;

use anyhow::{bail, Context};
use js_component_bindgen::{transpile, TranspileOpts};
use js_sys::Function;

use crate::{helpers::map_js_error, Result};

pub(crate) struct ComponentFiles {
    pub instantiate: Function,
    pub wasm_cores: Vec<(String, Vec<u8>)>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ComponentLoader {}

impl ComponentLoader {
    pub fn generate_files(bytes: &[u8]) -> Result<ComponentFiles> {
        let opts = TranspileOpts {
            instantiation: true,
            ..Default::default()
        };

        let transpiled = transpile(bytes, opts)?;
        let files = transpiled.files;

        let mut wasm_cores = Vec::<(String, Vec<u8>)>::new();
        let mut instantiate = Option::<Function>::None;

        for (name, bytes) in files.into_iter() {
            if name.ends_with(".wasm") {
                wasm_cores.push((name, bytes));
            } else if name.ends_with(".js") {
                let s = String::from_utf8_lossy(&bytes);
                tracing::debug!(s = %&*s, "js loader");
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
