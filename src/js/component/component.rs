use crate::Result;

pub struct Component {
    component: JsValue,
    instantiate: Function,
}

impl Component {
    pub async fn new(load_component: &str) -> Result<Self> {
        let component = js_sys::eval(load_component)?;
        let component = await_js_value(component).await?;

        let instantiate = Reflect::get(&component, &"instantiate".into())?;
        let instantiate: Function = instantiate.try_into()?;

        Ok(Self {
            component,
            instantiate
        })
    }
}
