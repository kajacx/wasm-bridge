use anyhow::Context;
use js_component_bindgen::{transpile, InstantiationMode, TranspileOpts};

use crate::Result;

pub(crate) struct ComponentFiles {
    pub(crate) main_core: Vec<u8>,
    pub(crate) wasi_core: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ComponentLoader {}

impl ComponentLoader {
    pub fn generate_files(bytes: &[u8]) -> Result<ComponentFiles> {
        let opts = TranspileOpts {
            instantiation: Some(InstantiationMode::Async),
            ..Default::default()
        };

        let transpiled = transpile(bytes, opts)?;
        let files = transpiled.files;

        let mut main_core = Option::<Vec<u8>>::None;
        let mut wasi_core = Option::<Vec<u8>>::None;
        let mut is_wasi = false;

        for (name, bytes) in files.into_iter() {
            if name.ends_with("core.wasm") {
                main_core = Some(bytes);
            } else if name.ends_with("core2.wasm") {
                wasi_core = Some(bytes);
            } else if name.ends_with("core4.wasm") {
                is_wasi = true;
            }
        }

        let main_core =
            main_core.context("JCO transpile should generate a main .core.wasm file")?;

        let wasi_core = wasi_core.filter(|_| is_wasi);

        Ok(ComponentFiles {
            main_core,
            wasi_core,
        })
    }
}
