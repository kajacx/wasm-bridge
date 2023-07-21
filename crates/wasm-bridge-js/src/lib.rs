mod engine;
pub use engine::*;

mod instance;
pub use instance::*;

mod module;
pub use module::*;

mod store;
pub use store::*;

mod typed_func;
pub use typed_func::*;

mod conversions;
pub use conversions::*;

mod linker;
pub use linker::*;

mod caller;
pub use caller::*;

mod config;
pub use config::*;

mod context;
pub use context::*;

pub type Error = anyhow::Error;
pub type Result<T, E = Error> = anyhow::Result<T, E>;

pub mod helpers;

#[cfg(feature = "component-model")]
pub mod component;

#[cfg(feature = "wasi")]
pub mod wasi;

pub use js_sys;
pub use wasm_bindgen;

#[allow(warnings)]
mod test {

    pub mod wasm_bridge {
        pub use crate::*;
    }

    pub struct TestWorld {
        add_three: wasm_bridge::component::Func,
    }
    const _: () = {
        use wasm_bridge::component::__internal::anyhow;
        impl TestWorld {
            /// Instantiates the provided `module` using the specified
            /// parameters, wrapping up the result in a structure that
            /// translates between wasm and the host.
            pub async fn instantiate_async<T: Send>(
                mut store: impl wasm_bridge::AsContextMut<Data = T>,
                component: &wasm_bridge::component::Component,
                linker: &wasm_bridge::component::Linker<T>,
            ) -> wasm_bridge::Result<(Self, wasm_bridge::component::Instance)> {
                let instance = linker.instantiate_async(&mut store, component).await?;
                Ok((Self::new(store, &instance)?, instance))
            }
            /// Instantiates a pre-instantiated module using the specified
            /// parameters, wrapping up the result in a structure that
            /// translates between wasm and the host.
            pub async fn instantiate_pre<T: Send>(
                mut store: impl wasm_bridge::AsContextMut<Data = T>,
                instance_pre: &wasm_bridge::component::InstancePre<T>,
            ) -> wasm_bridge::Result<(Self, wasm_bridge::component::Instance)> {
                let instance = instance_pre.instantiate_async(&mut store).await?;
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
                let add_three = __exports
                    .typed_func::<(i32,), (i32,)>("add-three")?
                    .func()
                    .clone();
                Ok(TestWorld { add_three })
            }
            pub async fn call_add_three<S: wasm_bridge::AsContextMut>(
                &self,
                mut store: S,
                arg0: i32,
            ) -> wasm_bridge::Result<i32>
            where
                <S as wasm_bridge::AsContext>::Data: Send,
            {
                let callee = unsafe {
                    wasm_bridge::component::TypedFunc::<(i32,), (i32,)>::new_unchecked(
                        self.add_three.clone(),
                    )
                };
                let (ret0,) = callee.call_async(store.as_context_mut(), (arg0,)).await?;
                callee.post_return_async(store.as_context_mut()).await?;
                Ok(ret0)
            }
        }
    };
}
