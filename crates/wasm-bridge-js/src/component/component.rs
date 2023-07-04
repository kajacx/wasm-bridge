use js_sys::{Function, Reflect};
use wasm_bindgen::JsValue;

use crate::Result;

use super::helpers::await_js_value;

pub struct Component {
    pub(crate) component: JsValue,
    pub(crate) instantiate: Function,
}

impl Component {
    pub async fn new(load_component: &str) -> Result<Self> {
        let component = js_sys::eval(load_component)?;
        let component: JsValue = await_js_value(component).await?;

        let instantiate = Reflect::get(&component, &"instantiate".into())?;
        let instantiate: Function = instantiate.into();

        Ok(Self { 
            component,
            instantiate,
        })
    }
}
