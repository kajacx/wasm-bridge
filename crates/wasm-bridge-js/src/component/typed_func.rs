use std::marker::PhantomData;

pub struct TypedFunc<Params, Return> {
    _phantom: PhantomData<dyn Fn(Params) -> Return>,
}
