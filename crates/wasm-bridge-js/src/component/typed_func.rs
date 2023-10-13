use std::marker::PhantomData;

use crate::{AsContextMut, FromJsValue, Result, Store, ToJsValue};

use super::{Func, Lower};

pub struct TypedFunc<Params, Return> {
    func: Func,
    _phantom: PhantomData<dyn Fn(Params) -> Return>,
}

impl<Params, Return> TypedFunc<Params, Return> {
    pub fn new(func: Func) -> Self {
        Self {
            func,
            _phantom: PhantomData,
        }
    }

    /// # Safety
    /// This function is 100% safe, it just needs to match wasmtime's API
    pub unsafe fn new_unchecked(func: Func) -> Self {
        Self::new(func)
    }

    pub fn func(&self) -> &Func {
        &self.func
    }

    pub fn call<T>(&self, store: &mut Store<T>, params: Params) -> Result<Return>
// where
        // Self: Callable<T>,
    {
        todo!()
        // Callable::call(&self, store, params)
        // let argument = params.to_function_args();
        // let result = self.func.function.apply(&JsValue::UNDEFINED, &argument);
        // Return::from_fn_result(&result)
    }

    pub fn call_async(&self, store: impl AsContextMut, params: Params) -> Result<Return>
    where
        Params: ToJsValue,
        Return: FromJsValue,
    {
        todo!()
        // self.call(store, params)
    }

    pub fn post_return(&self, _store: impl AsContextMut) -> Result<()> {
        Ok(())
    }

    pub fn post_return_async(&self, store: impl AsContextMut) -> Result<()> {
        self.post_return(store)
    }
}

pub trait Callable<T> {
    type Args;
    type Return;

    fn call(&self, store: &mut Store<T>, args: Self::Args) -> Result<Self::Return>;
}

impl<T, Arg, Ret> Callable<T> for TypedFunc<Arg, Ret>
where
    Arg: Lower,
{
    type Args = Arg;
    type Return = Ret;

    fn call(&self, store: &mut Store<T>, args: Self::Args) -> Result<Self::Return> {
        // let mut arg = MaybeUninit::uninit().into();

        // let ctx = LowerContext {};
        // args.lower(ctx);
        // self.func.function.call1(&JsValue::UNDEFINED, &arg);

        todo!()
    }
}
