#[proc_macro]
pub fn bindgen(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    r#"
    pub struct TestWorld {
        add_hello: wasm_bridge::component::Func,
    }
    const _: () = {
        use wasm_bridge::component::__internal::anyhow;
        impl TestWorld {
            /// Instantiates the provided `module` using the specified
            /// parameters, wrapping up the result in a structure that
            /// translates between wasm and the host.
            pub fn instantiate<T>(
                mut store: impl wasm_bridge::AsContextMut<Data = T>,
                component: &wasm_bridge::component::Component,
                linker: &wasm_bridge::component::Linker<T>,
            ) -> wasm_bridge::Result<(Self, wasm_bridge::component::Instance)> {
                let instance = linker.instantiate(&mut store, component)?;
                Ok((Self::new(store, &instance)?, instance))
            }
            /// Instantiates a pre-instantiated module using the specified
            /// parameters, wrapping up the result in a structure that
            /// translates between wasm and the host.
            pub fn instantiate_pre<T>(
                mut store: impl wasm_bridge::AsContextMut<Data = T>,
                instance_pre: &wasm_bridge::component::InstancePre<T>,
            ) -> wasm_bridge::Result<(Self, wasm_bridge::component::Instance)> {
                let instance = instance_pre.instantiate(&mut store)?;
                Ok((Self::new(store, &instance)?, instance))
            }
            /// Low-level creation wrapper for wrapping up the exports
            /// of the `instance` provided in this structure of wasm
            /// exports.
            ///
            /// This function will extract exports from the `instance`
            /// defined within `store` and wrap them all up in the
            /// returned structure which can be used to interact with
            /// the wasm module.
            pub fn new(
                mut store: impl wasm_bridge::AsContextMut,
                instance: &wasm_bridge::component::Instance,
            ) -> wasm_bridge::Result<Self> {
                let mut store = store.as_context_mut();
                let mut exports = instance.exports(&mut store);
                let mut __exports = exports.root();
                let add_hello = *__exports
                    .typed_func::<(&str,), (String,)>("add-hello")?
                    .func();
                Ok(TestWorld { add_hello })
            }
            pub fn call_add_hello<S: wasm_bridge::AsContextMut>(
                &self,
                mut store: S,
                arg0: &str,
            ) -> wasm_bridge::Result<String> {
                let callee = unsafe {
                    wasm_bridge::component::TypedFunc::<
                        (&str,),
                        (String,),
                    >::new_unchecked(self.add_hello)
                };
                let (ret0,) = callee.call(store.as_context_mut(), (arg0,))?;
                callee.post_return(store.as_context_mut())?;
                Ok(ret0)
            }
        }
    };
    const _: &str = "package wasm-bridge:protocol\n\nworld test-world {\n  export add-hello: func(text: string) -> string\n}\n";
    "#.parse().unwrap()
}
