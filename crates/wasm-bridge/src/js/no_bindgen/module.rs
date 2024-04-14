use std::borrow::Cow;

use crate::{helpers::map_js_error, *};
use anyhow::bail;
use js_sys::{Uint8Array, WebAssembly};
use wasm_bindgen_futures::JsFuture;

#[derive(Clone, Debug)]
pub struct Module {
    pub(crate) module: WebAssembly::Module,
}

impl Module {
    #[deprecated(
        since = "0.4.0",
        note = "Compiling a module synchronously can panic on the web, please use `new_safe` instead."
    )]
    pub fn new(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        let bytes = Self::resolve_bytes(bytes.as_ref())?;
        Self::from_bytes(&bytes)
    }

    pub async fn new_safe(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        let bytes = Self::resolve_bytes(bytes.as_ref())?;
        Self::from_bytes_async(&bytes).await
    }

    fn resolve_bytes(bytes: &[u8]) -> Result<Cow<[u8]>> {
        if bytes.is_empty() {
            bail!("Cannot create a module from empty bytes")
        }

        if let Ok(text) = std::str::from_utf8(bytes) {
            Ok(Cow::Owned(Self::parse_wat(text)?))
        } else {
            Ok(Cow::Borrowed(bytes))
        }
    }

    #[cfg(feature = "wat")]
    fn parse_wat(wat: &str) -> Result<Vec<u8>> {
        Ok(wat::parse_str(wat)?)
    }

    #[cfg(not(feature = "wat"))]
    fn parse_wat(_wat: &str) -> Result<Vec<u8>> {
        bail!("Module bytes are valid text, try enabling the 'wat' feature to parse it")
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let byte_array = Uint8Array::from(bytes);

        let module = WebAssembly::Module::new(&byte_array.into()).map_err(map_js_error(
            "Failed to synchronously compile bytes to a WASM module",
        ))?;

        Ok(Self { module })
    }

    async fn from_bytes_async(bytes: &[u8]) -> Result<Self> {
        let byte_array = Uint8Array::from(bytes);

        let promise = WebAssembly::compile(&byte_array);
        let module = JsFuture::from(promise).await.map_err(map_js_error(
            "Failed to asynchronously compile bytes to a WASM module",
        ))?;

        Ok(Self {
            module: module.into(),
        })
    }
}
