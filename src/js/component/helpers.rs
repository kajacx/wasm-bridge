pub(super) async fn await_js_value(value: JsValue) -> crate::Result<JsValue> {
    let as_promise = js_sys::Promise::try_from(value)?;
    JsFuture::from(as_promise).await
}
