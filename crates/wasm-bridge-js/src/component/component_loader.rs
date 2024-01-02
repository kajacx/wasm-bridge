use anyhow::Context;
use js_component_bindgen::{transpile, InstantiationMode, TranspileOpts};

use crate::Result;

pub(crate) struct ComponentFiles {
    pub(crate) core: Vec<u8>,
    pub(crate) core2: Option<Vec<u8>>,
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

        let mut core = Option::<Vec<u8>>::None;
        let mut core2 = Option::<Vec<u8>>::None;

        for (name, bytes) in files.into_iter() {
            if name.ends_with("core.wasm") {
                core = Some(bytes);
            } else if name.ends_with("core2.wasm") {
                core2 = Some(bytes);
            }
        }

        let core = core.context("JCO transpile should generate a main .core.wasm file")?;

        Ok(ComponentFiles { core, core2 })
    }
}
