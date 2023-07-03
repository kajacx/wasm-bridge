use std::fmt::{Debug, Display};

use wasm_bindgen::{JsValue, JsError};

#[derive(Debug)]
pub enum Error {
    InvalidWatText(String),
    ExportedFnNotFound(String),
    IncorrectNumOfArgs(String, u32, u32), // Name, expected, actual
    JsError(JsValue),
    // Other(String),
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Self::JsError(value)
    }
}

impl From<JsError> for Error {
    fn from(value: JsError) -> Self {
        Self::JsError(value.into())
    }
}

// impl From<String> for Error {
//     fn from(value: String) -> Self {
//         Self::Other(value)
//     }
// }

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidWatText(err) => write!(
                f,
                "Module bytes are valid text, but parsing it in wat format gave error: {err}"
            ),
            Error::ExportedFnNotFound(name) => {
                write!(f, "Exported fn `{name}` not found in the module")
            }
            Error::IncorrectNumOfArgs(name, expected, actual) => write!(
                f,
                "Expected `{name}` to have {expected} arguments, but it has {actual} instead"
            ),
            Error::JsError(value) => write!(f, "{value:?}"),
            // Error::Other(value) => write!(f, "Other error: {value:?}"),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
