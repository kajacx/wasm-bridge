use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Module {}

impl Module {
    pub fn new(_engine: &Engine, _bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        Ok(Self {})
    }
}
