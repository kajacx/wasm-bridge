use crate::*;
use js_sys::{Uint8Array, WebAssembly};

#[derive(Clone, Debug)]
pub struct Module {
    pub(crate) module: WebAssembly::Module,
}

impl Module {
    pub fn new(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let bytes = bytes.as_ref();
        Self::from_bytes(bytes.as_ref()).or_else(|err| Self::from_wat(bytes, err))
    }

    fn from_wat(wat: &[u8], err: Error) -> Result<Self, Error> {
        let bytes = wat::parse_bytes(wat).map_err(|_| err)?;

        Self::from_bytes(&bytes)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        // TODO: view might be faster than from, but its unsafe
        // Uint8Array::view(bytes.as_ref());

        let byte_array = Uint8Array::from(bytes.as_ref());
        let module = WebAssembly::Module::new(&byte_array.into())?;
        Ok(Self { module })
    }
}
