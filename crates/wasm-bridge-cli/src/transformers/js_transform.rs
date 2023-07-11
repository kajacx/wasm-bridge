use anyhow::Context;
use regex::Regex;

use crate::{transformers::Transformer, zipper::ZippableFile};

pub fn js_transformer(keep_original: bool) -> Transformer {
    Box::new(move |files| {
        let (index, file) = files
            .iter()
            .enumerate()
            .find(|(_, file)| file.name.ends_with(".js"))
            .context("Source folder is missing the component .js file. Make sure to specify the folder created by jco transpile.")?;

        let text = std::str::from_utf8(&file.content).context("Valid UTF-8 bytes in .js file")?;
        let text = modify_js(text);

        files.push(ZippableFile {
            name: "sync_component.js".into(),
            content: text.into_bytes(),
        });

        if !keep_original {
            files.remove(index);

            // Remove TS bindings
            files.retain(|file| !file.name.ends_with(".d.ts"));
        }

        Ok(())
    })
}

fn modify_js(text: &str) -> String {
    // function signature
    let text = text.replace("export async function", "function");

    // remove all awaits
    let text = text.replace("await ", "");

    // remove Promise.all call
    let regex = Regex::new(".*Promise\\.all.*").unwrap();
    let text = regex.replace_all(&text, "");

    // Final update
    let text = format!("(() => {{\n{text}\nreturn instantiate;\n}})()\n");

    text
}
