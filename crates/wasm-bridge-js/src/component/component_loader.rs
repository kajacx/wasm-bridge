use anyhow::Context;
use js_component_bindgen::{transpile, InstantiationMode, TranspileOpts};

use crate::Result;

pub(crate) struct ComponentFiles {
    pub(crate) core: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ComponentLoader {}

impl ComponentLoader {
    pub fn generate_files(bytes: &[u8]) -> Result<ComponentFiles> {
        let opts = TranspileOpts {
            instantiation: Some(InstantiationMode::Async), // TODO: check this
            ..Default::default()
        };

        let transpiled = transpile(bytes, opts)?;
        let files = transpiled.files;

        let mut core = Option::<Vec<u8>>::None;

        for (name, bytes) in files.into_iter() {
            if name.ends_with("core.wasm") {
                // TODO: detect multiple cores
                core = Some(bytes);
            }
        }

        let core = core.context("JCO transpile should generate a main .core.wasm file")?;

        Ok(ComponentFiles { core })
    }
}
