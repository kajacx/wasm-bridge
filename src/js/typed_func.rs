use crate::*;

use std::marker::PhantomData;

pub struct TypedFunc<Params, Results> {
    _phantom: PhantomData<fn(params: Params) -> Results>,
}

impl<Params, Results> TypedFunc<Params, Results> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    pub fn call(&self, _store: &Store<()>, _params: Params) -> Result<Results, Error> {
        todo!()
    }
}
