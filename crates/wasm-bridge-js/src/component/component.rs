use js_sys::{Function, Reflect};
use wasm_bindgen::JsValue;

use crate::{AsContext, Engine, Result};

pub struct Component {
    pub(crate) component: JsValue,
    pub(crate) instantiate: Function,
}

impl Component {
    pub fn new(_engine: &Engine, _bytes: &[u8]) -> Result<Self> {
        // let component = js_sys::eval(load_component)?;
        // let component: JsValue = await_js_value(component).await?;

        // let instantiate = Reflect::get(&component, &"instantiate".into())?;
        // let instantiate: Function = instantiate.into();

        // Ok(Self {
        //     component,
        //     instantiate,
        // })

        todo!()
    }
}
