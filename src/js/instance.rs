use crate::*;

pub struct Instance {}

impl Instance {
    pub fn new(
        _store: &mut Store<()>,
        _module: &Module,
        _: impl AsRef<[()]>,
    ) -> Result<Self, Error> {
        Ok(Self {})
    }
}
