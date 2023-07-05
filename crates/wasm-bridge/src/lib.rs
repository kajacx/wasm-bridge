#[cfg(not(target_arch = "wasm32"))]
pub use wasmtime::*;

#[cfg(target_arch = "wasm32")]
pub use wasm_bridge_js::*;

#[test]
fn test() {
    panic!("To test `wasm-bridge`, run the `run_all_tests.sh` script from the `tests` folder.");
}

// TODO: for testing only, remove later
#[cfg(all(not(target_arch = "wasm32"), feature = "component-model"))]
mod host {
    use std::error::Error;

    use wasmtime::component::Component;
    pub struct TestWorld {
        add_hello: wasmtime::component::Func,
    }
    const _: () = {
        use wasmtime::component::__internal::anyhow;
        impl TestWorld {
            pub fn instantiate<T>(
                mut store: impl wasmtime::AsContextMut<Data = T>,
                component: &wasmtime::component::Component,
                linker: &wasmtime::component::Linker<T>,
            ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
                let instance = linker.instantiate(&mut store, component)?;
                Ok((Self::new(store, &instance)?, instance))
            }

            pub fn instantiate_pre<T>(
                mut store: impl wasmtime::AsContextMut<Data = T>,
                instance_pre: &wasmtime::component::InstancePre<T>,
            ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
                let instance = instance_pre.instantiate(&mut store)?;
                Ok((Self::new(store, &instance)?, instance))
            }

            pub fn new(
                mut store: impl wasmtime::AsContextMut,
                instance: &wasmtime::component::Instance,
            ) -> wasmtime::Result<Self> {
                let mut store = store.as_context_mut();
                let mut exports = instance.exports(&mut store);
                let mut __exports = exports.root();
                let add_hello = *__exports
                    .typed_func::<(&str,), (String,)>("add-hello")?
                    .func();
                Ok(TestWorld { add_hello })
            }
            pub fn call_add_hello<S: wasmtime::AsContextMut>(
                &self,
                mut store: S,
                arg0: &str,
            ) -> wasmtime::Result<String> {
                let callee = unsafe {
                    wasmtime::component::TypedFunc::<(&str,), (String,)>::new_unchecked(
                        self.add_hello,
                    )
                };
                let (ret0,) = callee.call(store.as_context_mut(), (arg0,))?;
                callee.post_return(store.as_context_mut())?;
                Ok(ret0)
            }
        }
    };
    const _: &str = "package wasm-bridge:protocol\n\nworld test-world {\n  export add-hello: func(text: string) -> string\n}\n";
    fn main() {
        //let component = Component::new(engine, bytes);
    }
}
