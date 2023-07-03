use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

pub(super) async fn await_js_value(value: JsValue) -> crate::Result<JsValue> {
    let as_promise = js_sys::Promise::from(value); // TODO: try_from?
    Ok(JsFuture::from(as_promise).await?)
}
