use super::*;
use crate::Result;

#[derive(Debug, Clone, Default)]
pub struct ComponentLoader {}

impl ComponentLoader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile_component(self, bytes: &[u8]) -> Result<Component> {
        let opts = wasm_bridge_jco::TranspileOpts {
            instantiation: true,
            ..Default::default()
        };

        let result = wasm_bridge_jco::transpile(bytes, opts)?;
        let mut files = result.files;

        for (name, bytes) in files.iter_mut() {
            if name.ends_with(".js") {
                *name = "sync_component.js".into();
                *bytes = modify_js_bytes(bytes)?;
            }
        }

        Component::from_files(files)
    }
}

fn modify_js_bytes(bytes: &[u8]) -> Result<Vec<u8>> {
    let text = std::str::from_utf8(bytes)?;
    let text = modify_js(text);
    Ok(text.into())
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
