use std::fmt::{Debug, Display};

use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    JsError(JsValue),
}

impl From<JsValue> for Error {
    fn from(v: JsValue) -> Self {
        Self::JsError(v)
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::JsError(value) => write!(f, "{:?}", value),
        }
    }
}
