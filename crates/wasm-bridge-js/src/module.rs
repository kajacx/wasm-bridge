use crate::*;
use js_sys::{Uint8Array, WebAssembly};

#[derive(Clone, Debug)]
pub struct Module {
    pub(crate) module: WebAssembly::Module,
}

impl Module {
    pub fn new(_engine: &Engine, bytes: impl AsRef<[u8]>) -> Result<Self> {
        let bytes = bytes.as_ref();
        Self::from_bytes(bytes).or_else(|err| Self::from_wat(bytes, err))
    }

    #[cfg(feature = "wat")]
    fn from_wat(wat: &[u8], original_err: Error) -> Result<Self> {
        // If it's not text, give back the original error, it's probably more useful
        let text: &str = std::str::from_utf8(wat).map_err(move |_| original_err)?;

        let bytes =
            wat::parse_str(text).map_err(|err| Error::InvalidWatText(format!("{err:?}")))?;

        Self::from_bytes(&bytes)
    }

    #[cfg(not(feature = "wat"))]
    fn from_wat(_wat: &[u8], original_err: Error) -> Result<Self> {
        // TODO: Add the "enable wat" warning if _wat is valid utf-8?
        Err(original_err)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        // TODO: view might be faster than from, but its unsafe
        // Uint8Array::view(bytes.as_ref());

        let byte_array = Uint8Array::from(bytes);
        let module = WebAssembly::Module::new(&byte_array.into())?;
        Ok(Self { module })
    }
}
