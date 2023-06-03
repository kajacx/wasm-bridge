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

impl Instance {
    pub fn get_typed_func<Params, Results>(
        &self,
        _store: &mut Store<()>,
        _name: &str,
    ) -> Result<TypedFunc<Params, Results>, Error> {
        todo!()
    }
}
