use std::path::PathBuf;

use anyhow::{bail, Context};

use crate::reader::read_path_buf;

use super::Transformer;

pub fn add_original_file(file: PathBuf) -> Transformer {
    Box::new(move |files| {
        if !file.is_file() {
            bail!("Pass the WASM component file name as the --universal parameter");
        }

        let mut component = read_path_buf(&file)?
            .into_iter()
            .next()
            .context("Should read original file")?;

        component.name = "original_component.wasm".into();

        files.push(component);
        Ok(())
    })
}
