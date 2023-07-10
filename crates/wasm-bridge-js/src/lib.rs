mod engine;
pub use engine::*;

mod error;
pub use error::*;

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

pub(crate) mod helpers;

#[cfg(feature = "component-model")]
pub mod component;

mod test {
    mod wasm_bridge {
        pub use crate::*;
    }

    pub struct TestWorld {
        add_abc: wasm_bridge::component::Func,
        add_all_and_one: wasm_bridge::component::Func,
        add_hello: wasm_bridge::component::Func,
        add_numbers: wasm_bridge::component::Func,
        add_sub_one: wasm_bridge::component::Func,
        add_sub_twenty: wasm_bridge::component::Func,
        increment_twice: wasm_bridge::component::Func,
        push_numbers: wasm_bridge::component::Func,
        sqrt: wasm_bridge::component::Func,
    }
    pub trait TestWorldImports {
        fn add_b(&mut self, text: String) -> wasm_bridge::Result<String>;
        fn add_numbers_import(&mut self, a: i32, b: i32) -> wasm_bridge::Result<i32>;
        fn increment(&mut self) -> wasm_bridge::Result<()>;
        fn add_all(
            &mut self,
            a: i32,
            b: i64,
            c: u32,
            d: u64,
            e: f32,
            f: f64,
            g: String,
        ) -> wasm_bridge::Result<f64>;
        fn add_sub_two(&mut self, num: i32) -> wasm_bridge::Result<(i32, i32)>;
        fn add_sub_ten(&mut self, num: i32) -> wasm_bridge::Result<(i32, i32)>;
        fn sqrt_import(&mut self, num: Option<f64>) -> wasm_bridge::Result<Option<f64>>;
    }
    const _: () = {
        use wasm_bridge::component::__internal::anyhow;
        impl TestWorld {
            pub fn add_to_linker<T: 'static, U>(
                linker: &mut wasm_bridge::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasm_bridge::Result<()>
            where
                U: TestWorldImports,
            {
                Self::add_root_to_linker(linker, get)?;
                Ok(())
            }
            pub fn add_root_to_linker<T: 'static, U>(
                linker: &mut wasm_bridge::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasm_bridge::Result<()>
            where
                U: TestWorldImports,
            {
                let mut linker = linker.root();
                linker.func_wrap(
                    "add-b",
                    move |mut caller: wasm_bridge::StoreContextMut<'_, T>, (arg0,): (String,)| {
                        let host = get(&mut caller);
                        let r = host.add_b(arg0);
                        Ok((r?,))
                    },
                )?;
                linker.func_wrap(
                    "add-numbers-import",
                    move |mut caller: wasm_bridge::StoreContextMut<'_, T>,
                          (arg0, arg1): (i32, i32)| {
                        let host = get(&mut caller);
                        let r = host.add_numbers_import(arg0, arg1);
                        Ok((r?,))
                    },
                )?;
                linker.func_wrap(
                    "increment",
                    move |mut caller: wasm_bridge::StoreContextMut<'_, T>, (): ()| {
                        let host = get(&mut caller);
                        let r = host.increment();
                        r
                    },
                )?;
                linker.func_wrap(
                    "add-all",
                    move |mut caller: wasm_bridge::StoreContextMut<'_, T>,
                          (arg0, arg1, arg2, arg3, arg4, arg5, arg6): (
                        i32,
                        i64,
                        u32,
                        u64,
                        f32,
                        f64,
                        String,
                    )| {
                        let host = get(&mut caller);
                        let r = host.add_all(arg0, arg1, arg2, arg3, arg4, arg5, arg6);
                        Ok((r?,))
                    },
                )?;
                linker.func_wrap(
                    "add-sub-two",
                    move |mut caller: wasm_bridge::StoreContextMut<'_, T>, (arg0,): (i32,)| {
                        let host = get(&mut caller);
                        let r = host.add_sub_two(arg0);
                        Ok((r?,))
                    },
                )?;
                linker.func_wrap(
                    "add-sub-ten",
                    move |mut caller: wasm_bridge::StoreContextMut<'_, T>, (arg0,): (i32,)| {
                        let host = get(&mut caller);
                        let r = host.add_sub_ten(arg0);
                        r
                    },
                )?;
                linker.func_wrap(
                    "sqrt-import",
                    move |mut caller: wasm_bridge::StoreContextMut<'_, T>,
                          (arg0,): (Option<f64>,)| {
                        let host = get(&mut caller);
                        let r = host.sqrt_import(arg0);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
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
                let add_abc = __exports
                    .typed_func::<(&str,), (String,)>("add-abc")?
                    .func()
                    .clone();
                let add_all_and_one = __exports
                    .typed_func::<(i32, i64, u32, u64, f32, f64, &str), (f64,)>("add-all-and-one")?
                    .func()
                    .clone();
                let add_hello = __exports
                    .typed_func::<(&str,), (String,)>("add-hello")?
                    .func()
                    .clone();
                let add_numbers = __exports
                    .typed_func::<(i32, i32), (i32,)>("add-numbers")?
                    .func()
                    .clone();
                let add_sub_one = __exports
                    .typed_func::<(i32,), ((i32, i32),)>("add-sub-one")?
                    .func()
                    .clone();
                let add_sub_twenty = __exports
                    .typed_func::<(i32,), (i32, i32)>("add-sub-twenty")?
                    .func()
                    .clone();
                let increment_twice = __exports
                    .typed_func::<(), ()>("increment-twice")?
                    .func()
                    .clone();
                let push_numbers = __exports
                    .typed_func::<(&[i32], i32, i32), (Vec<i32>,)>("push-numbers")?
                    .func()
                    .clone();
                let sqrt = __exports
                    .typed_func::<(Option<f64>,), (Option<f64>,)>("sqrt")?
                    .func()
                    .clone();
                Ok(TestWorld {
                    add_abc,
                    add_all_and_one,
                    add_hello,
                    add_numbers,
                    add_sub_one,
                    add_sub_twenty,
                    increment_twice,
                    push_numbers,
                    sqrt,
                })
            }
            pub fn call_add_hello<S: wasm_bridge::AsContextMut>(
                &self,
                mut store: S,
                arg0: &str,
            ) -> wasm_bridge::Result<String> {
                let callee = unsafe {
                    wasm_bridge::component::TypedFunc::<(&str,), (String,)>::new_unchecked(
                        self.add_hello.clone(),
                    )
                };
                let (ret0,) = callee.call(store.as_context_mut(), (arg0,))?;
                callee.post_return(store.as_context_mut())?;
                Ok(ret0)
            }
            pub fn call_add_abc<S: wasm_bridge::AsContextMut>(
                &self,
                mut store: S,
                arg0: &str,
            ) -> wasm_bridge::Result<String> {
                let callee = unsafe {
                    wasm_bridge::component::TypedFunc::<(&str,), (String,)>::new_unchecked(
                        self.add_abc.clone(),
                    )
                };
                let (ret0,) = callee.call(store.as_context_mut(), (arg0,))?;
                callee.post_return(store.as_context_mut())?;
                Ok(ret0)
            }
            pub fn call_add_numbers<S: wasm_bridge::AsContextMut>(
                &self,
                mut store: S,
                arg0: i32,
                arg1: i32,
            ) -> wasm_bridge::Result<i32> {
                let callee = unsafe {
                    wasm_bridge::component::TypedFunc::<(i32, i32), (i32,)>::new_unchecked(
                        self.add_numbers.clone(),
                    )
                };
                let (ret0,) = callee.call(store.as_context_mut(), (arg0, arg1))?;
                callee.post_return(store.as_context_mut())?;
                Ok(ret0)
            }
            pub fn call_increment_twice<S: wasm_bridge::AsContextMut>(
                &self,
                mut store: S,
            ) -> wasm_bridge::Result<()> {
                let callee = unsafe {
                    wasm_bridge::component::TypedFunc::<(), ()>::new_unchecked(
                        self.increment_twice.clone(),
                    )
                };
                let () = callee.call(store.as_context_mut(), ())?;
                callee.post_return(store.as_context_mut())?;
                Ok(())
            }
            pub fn call_add_all_and_one<S: wasm_bridge::AsContextMut>(
                &self,
                mut store: S,
                arg0: i32,
                arg1: i64,
                arg2: u32,
                arg3: u64,
                arg4: f32,
                arg5: f64,
                arg6: &str,
            ) -> wasm_bridge::Result<f64> {
                let callee = unsafe {
                    wasm_bridge :: component :: TypedFunc :: <
                          (i32, i64, u32, u64, f32, f64, & str,), (f64,) > ::
                          new_unchecked(self.add_all_and_one.clone())
                };
                let (ret0,) = callee.call(
                    store.as_context_mut(),
                    (arg0, arg1, arg2, arg3, arg4, arg5, arg6),
                )?;
                callee.post_return(store.as_context_mut())?;
                Ok(ret0)
            }
            pub fn call_add_sub_one<S: wasm_bridge::AsContextMut>(
                &self,
                mut store: S,
                arg0: i32,
            ) -> wasm_bridge::Result<(i32, i32)> {
                let callee = unsafe {
                    wasm_bridge::component::TypedFunc::<(i32,), ((i32, i32),)>::new_unchecked(
                        self.add_sub_one.clone(),
                    )
                };
                let (ret0,) = callee.call(store.as_context_mut(), (arg0,))?;
                callee.post_return(store.as_context_mut())?;
                Ok(ret0)
            }
            pub fn call_add_sub_twenty<S: wasm_bridge::AsContextMut>(
                &self,
                mut store: S,
                arg0: i32,
            ) -> wasm_bridge::Result<(i32, i32)> {
                let callee = unsafe {
                    wasm_bridge::component::TypedFunc::<(i32,), (i32, i32)>::new_unchecked(
                        self.add_sub_twenty.clone(),
                    )
                };
                let (ret0, ret1) = callee.call(store.as_context_mut(), (arg0,))?;
                callee.post_return(store.as_context_mut())?;
                Ok((ret0, ret1))
            }
            pub fn call_sqrt<S: wasm_bridge::AsContextMut>(
                &self,
                mut store: S,
                arg0: Option<f64>,
            ) -> wasm_bridge::Result<Option<f64>> {
                let callee = unsafe {
                    wasm_bridge::component::TypedFunc::<(Option<f64>,), (Option<f64>,)>::new_unchecked(
                    self.sqrt.clone(),
                )
                };
                let (ret0,) = callee.call(store.as_context_mut(), (arg0,))?;
                callee.post_return(store.as_context_mut())?;
                Ok(ret0)
            }
            pub fn call_push_numbers<S: wasm_bridge::AsContextMut>(
                &self,
                mut store: S,
                arg0: &[i32],
                arg1: i32,
                arg2: i32,
            ) -> wasm_bridge::Result<Vec<i32>> {
                let callee = unsafe {
                    wasm_bridge::component::TypedFunc::<(&[i32], i32, i32), (Vec<i32>,)>::new_unchecked(
                    self.push_numbers.clone(),
                )
                };
                let (ret0,) = callee.call(store.as_context_mut(), (arg0, arg1, arg2))?;
                callee.post_return(store.as_context_mut())?;
                Ok(ret0)
            }
        }
    };

    use crate::{
        component::{Component, Linker},
        Config, Engine, Result, Store,
    };

    struct HostData {
        number: i32,
    }

    impl TestWorldImports for HostData {
        fn add_b(&mut self, text: String) -> Result<String> {
            Ok(text + "b")
        }

        fn add_numbers_import(&mut self, a: i32, b: i32) -> Result<i32> {
            Ok(a + b)
        }

        fn increment(&mut self) -> Result<()> {
            self.number += 1;
            Ok(())
        }

        fn add_sub_two(&mut self, num: i32) -> Result<(i32, i32)> {
            Ok((num + 2, num - 2))
        }

        fn add_sub_ten(&mut self, num: i32) -> Result<(i32, i32)> {
            Ok((num + 10, num - 10))
        }

        fn add_all(
            &mut self,
            a: i32,
            b: i64,
            c: u32,
            d: u64,
            e: f32,
            f: f64,
            g: String,
        ) -> Result<f64> {
            Ok(
                a as f64
                    + b as f64
                    + c as f64
                    + d as f64
                    + e as f64
                    + f
                    + g.parse::<f64>().unwrap(),
            )
        }

        fn sqrt_import(&mut self, num: Option<f64>) -> Result<Option<f64>> {
            Ok(match num {
                Some(value) if value >= 0.0 => Some(value.sqrt()),
                _ => None,
            })
        }
    }

    pub fn run_test(component_bytes: &[u8]) -> Result<()> {
        let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = Engine::new(&config)?;
        let mut store = Store::new(&engine, HostData { number: 0 });

        let component = Component::new(&store.engine(), &component_bytes)?;

        let mut linker = Linker::new(store.engine());
        TestWorld::add_to_linker(&mut linker, |data| data)?;

        let (instance, _) = TestWorld::instantiate(&mut store, &component, &linker)?;

        let result = instance.call_add_hello(&mut store, "world")?;
        assert_eq!(result, "Hello world");

        let result = instance.call_add_abc(&mut store, "Hello ")?;
        assert_eq!(result, "Hello abc");

        let result = instance.call_add_numbers(&mut store, 5, 6)?;
        assert_eq!(result, 11);

        instance.call_increment_twice(&mut store)?;
        assert_eq!(store.data().number, 2);

        let result = instance.call_add_all_and_one(
            &mut store, 10i32, 20i64, 30u32, 40u64, 50.25f32, 60.25f64, "70",
        )?;
        assert_eq!(
            result,
            10.0 + 20.0 + 30.0 + 40.0 + 50.25 + 60.25 + 70.0 + 1.0
        );

        let result = instance.call_add_sub_one(&mut store, 5)?;
        assert_eq!(result, (6, 4));

        let result = instance.call_add_sub_twenty(&mut store, 5)?;
        assert_eq!(result, (25, -15));

        let result = instance.call_sqrt(&mut store, Some(16.0))?;
        assert_eq!(result, Some(4.0));
        let result = instance.call_sqrt(&mut store, Some(-16.0))?;
        assert_eq!(result, None);
        let result = instance.call_sqrt(&mut store, None)?;
        assert_eq!(result, None);

        // multiple references to data
        let data1 = store.data();
        let data2 = store.data();
        assert_eq!(data1.number, data2.number);
        drop(data1);
        drop(data2);

        let result = instance.call_push_numbers(&mut store, &[1, 2], 3, 4)?;
        assert_eq!(result, vec![1, 2, 3, 4]);

        Ok(())
    }
}
