use std::fmt::{Debug, Display};

use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    JsError(JsValue),
    IncorrectNumOfArgs(String, u32, u32), // Name, expected, actual
    Other(String),
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Self::JsError(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::JsError(value) => write!(f, "{value:?}"),
            Error::IncorrectNumOfArgs(name, expected, actual) => write!(
                f,
                "Expected `{name}` to have {expected} arguments, but it has {actual} instead"
            ),
            Error::Other(value) => write!(f, "Other error: {value:?}"),
        }
    }
}
