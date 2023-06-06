use crate::*;
use js_sys::{Uint8Array, WebAssembly};

#[derive(Clone, Debug)]
pub struct Module {
    pub(crate) module: WebAssembly::Module,
}

impl Module {
    pub fn new(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        // TODO: view might be faster than from, but its unsafe
        // Uint8Array::view(bytes.as_ref());
        let bytes = Uint8Array::from(bytes.as_ref());
        let module = WebAssembly::Module::new(&bytes.into()).expect("TODO: new module");
        Ok(Self { module })
    }
}
