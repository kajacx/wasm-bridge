use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Engine {}

impl Engine {
    pub fn new(_: &Config) -> Result<Self, Error> {
        Ok(Self {})
    }
}
