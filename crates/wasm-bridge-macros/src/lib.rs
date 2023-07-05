#[proc_macro]
pub fn bindgen(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let stream = wasmtime_component_macro::bindgen!(stream);

    // // Modify the stream
    eprintln!("{stream:?}");

    stream
}
