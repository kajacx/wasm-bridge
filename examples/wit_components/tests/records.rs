use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

#[derive(wasm_bridge::component::ComponentType)]
#[derive(wasm_bridge::component::Lift)]
#[derive(wasm_bridge::component::Lower)]
#[component(record)]
#[derive(Copy,Clone)]
pub struct Item {
  #[component(name = "a")]
  pub a:u32, #[component(name = "b")]
  pub b:u64,
}
impl core::fmt::Debug for Item {
  fn fmt(&self,f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("Item").field("a", &self.a).field("b", &self.b).finish()
  }

  }
const _:() = {
  assert!(16 =  =  <Item as wasm_bridge::component::ComponentType>::SIZE32);
  assert!(8 =  =  <Item as wasm_bridge::component::ComponentType>::ALIGN32);
};
pub struct Records {
  run:wasm_bridge::component::Func,run_args:wasm_bridge::component::Func,
}
pub trait RecordsImports {
  fn send_item(&mut self,item:Item,) -> wasm_bridge::Result<()>;

  }const _:() = {
  use wasm_bridge::component::__internal::anyhow;
  impl Records {
    pub fn add_to_linker<T,U>(linker: &mut wasm_bridge::component::Linker<T>,get:impl Fn(&mut T) ->  &mut U+Send+Sync+Copy+'static,) -> wasm_bridge::Result<()>where U:RecordsImports,{
      Self::add_root_to_linker(linker,get)?;
      Ok(())
    }
    pub fn add_root_to_linker<T,U>(linker: &mut wasm_bridge::component::Linker<T>,get:impl Fn(&mut T) ->  &mut U+Send+Sync+Copy+'static,) -> wasm_bridge::Result<()>where U:RecordsImports {
      let mut linker = linker.root();
      linker.func_wrap("send-item",move|mut caller:wasm_bridge::StoreContextMut<'_,T>,(arg0,):(Item,)|{
        let host = get(caller.data_mut());
        let r = host.send_item(arg0,);
        r
      })?;
      Ok(())
    }
    #[doc = " Instantiates the provided `module` using the specified"]
    #[doc = " parameters, wrapping up the result in a structure that"]
    #[doc = " translates between wasm and the host."]
    pub fn instantiate<T>(mut store:impl wasm_bridge::AsContextMut<Data = T>,component: &wasm_bridge::component::Component,linker: &wasm_bridge::component::Linker<T>,) -> wasm_bridge::Result<(Self,wasm_bridge::component::Instance)>{
      let instance = linker.instantiate(&mut store,component)?;
      Ok((Self::new(store, &instance)?,instance))
    }
    #[doc = " Instantiates a pre-instantiated module using the specified"]
    #[doc = " parameters, wrapping up the result in a structure that"]
    #[doc = " translates between wasm and the host."]
    pub fn instantiate_pre<T>(mut store:impl wasm_bridge::AsContextMut<Data = T>,instance_pre: &wasm_bridge::component::InstancePre<T>,) -> wasm_bridge::Result<(Self,wasm_bridge::component::Instance)>{
      let instance = instance_pre.instantiate(&mut store)?;
      Ok((Self::new(store, &instance)?,instance))
    }
    #[doc = " Low-level creation wrapper for wrapping up the exports"]
    #[doc = " of the `instance` provided in this structure of wasm"]
    #[doc = " exports."]
    #[doc = ""]
    #[doc = " This function will extract exports from the `instance`"]
    #[doc = " defined within `store` and wrap them all up in the"]
    #[doc = " returned structure which can be used to interact with"]
    #[doc = " the wasm module."]
    pub fn new(mut store:impl wasm_bridge::AsContextMut,instance: &wasm_bridge::component::Instance,) -> wasm_bridge::Result<Self>{
      let mut store = store.as_context_mut();
      let mut exports = instance.exports(&mut store);
      let mut __exports = exports.root();
      let run =  *__exports.typed_func::<(u32,),()>("run")?.func();
      let run_args =  *__exports.typed_func::<(&[u32],),()>("run-args")?.func();
      Ok(Records {
        run,run_args,
      })
    }
    pub fn call_run<S:wasm_bridge::AsContextMut>(&self,mut store:S,arg0:u32,) -> wasm_bridge::Result<()>{
      let callee = unsafe {
        wasm_bridge::component::TypedFunc::<(u32,),()>::new_unchecked(self.run)
      };
      let() = callee.call(store.as_context_mut(),(arg0,))?;
      callee.post_return(store.as_context_mut())?;
      Ok(())
    }
    pub fn call_run_args<S:wasm_bridge::AsContextMut>(&self,mut store:S,arg0: &[u32],) -> wasm_bridge::Result<()>{
      let callee = unsafe {
        wasm_bridge::component::TypedFunc::<(&[u32],),()>::new_unchecked(self.run_args)
      };
      let() = callee.call(store.as_context_mut(),(arg0,))?;
      callee.post_return(store.as_context_mut())?;
      Ok(())
    }
  
    }

  };
const _: &str = include_str!(r#"/Users/teiroberts/dev/wasm-bridge/examples/wit_components/./records.wit"#);

#[derive(Default, Debug, Clone)]
struct Host {
    // messages: Vec<Item>,
}

// impl RecordsImports for Host {
// fn send_item(&mut self, item: Item) -> wasm_bridge::Result<()> {
//     self.messages.push(item);

//     Ok(())
// }

// fn send_items(&mut self, items: Vec<Item>) -> wasm_bridge::Result<()> {
//     self.messages.extend(items);

//     Ok(())
// }
// }

// impl PartialEq for Item {
//     fn eq(&self, other: &Self) -> bool {
//         self.a == other.a && self.b == other.b
//     }
// }

#[test]
#[wasm_bindgen_test]
fn records() {
    wit_components_tests::setup_tracing();
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, Host::default());

    let component = Component::new(store.engine(), GUEST_BYTES).unwrap();

    let mut linker = Linker::new(store.engine());
    // Records::add_to_linker(&mut linker, |data| data).unwrap();

    let (instance, _) = Records::instantiate(&mut store, &component, &linker).unwrap();

    instance.call_run(&mut store, 1).unwrap();

    // let data = store.data();
    // assert_eq!(data.messages, &[Item { a: 1, b: 2 }]);

    // assert_eq!(result, ["sword", "shield", "apple"]);
    // let result = instance.call_get_counts(&mut store).unwrap();
    // assert_eq!(result, [1, 2, 5]);
}

const GUEST_BYTES: &[u8] =
    include_bytes!("../../../target/wasm32-unknown-unknown/debug/records_guest.wasm");
