use js_sys::Function;
use wasm_bindgen::JsValue;

use crate::{Engine, Result};

pub struct Component {
    pub(crate) _component: JsValue,
    pub(crate) _instantiate: Function,
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
