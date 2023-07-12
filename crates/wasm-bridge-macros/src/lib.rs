use std::str::FromStr;

use regex::Regex;

mod bindgen;
mod component;
mod original;

#[proc_macro_derive(Lift, attributes(component))]
pub fn lift(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    replace_namespace(original::lift(input))
}

#[proc_macro_derive(Lower, attributes(component))]
pub fn lower(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    replace_namespace(original::lower(input))
}

#[proc_macro_derive(ComponentType, attributes(component))]
pub fn component_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    replace_namespace(original::component_type(input))
}

#[proc_macro]
pub fn flags(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    replace_namespace(original::flags(input))
}

#[proc_macro]
pub fn bindgen_sys(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let as_string = replace_namespace(original::bindgen(input));

    // TODO: this should not be needed
    let as_string = format!("mod wasmtime {{ pub use wasm_bridge::*; }}\n\n{as_string}");

    proc_macro::TokenStream::from_str(&as_string).unwrap()
}

#[proc_macro]
pub fn bindgen_js(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let as_string = replace_namespace(original::bindgen(input)).to_string();

    // Clone exported function
    let regex = Regex::new("\\*\\s*__exports\\.typed_func([^?]*)\\?\\.func\\(\\)").unwrap();
    let as_string = regex.replace_all(&as_string, "__exports.typed_func$1?.func().clone()");

    // Clone "inner" function
    let regex = Regex::new("new_unchecked\\(self\\.([^)]*)\\)").unwrap();
    let as_string = regex.replace_all(&as_string, "new_unchecked(self.$1.clone())");

    // Workaround to get data reference
    let regex = Regex::new("let host = get\\(caller\\.data_mut\\(\\)\\)\\s*;").unwrap();
    let as_string = regex.replace_all(&as_string, "let host = get(&mut caller);\n");

    // TODO: these static bounds are not great
    let regex = Regex::new("add_to_linker\\s*<\\s*T").unwrap();
    let as_string = regex.replace_all(&as_string, "add_to_linker<T: 'static");

    let regex = Regex::new("add_root_to_linker\\s*<\\s*T").unwrap();
    let as_string = regex.replace_all(&as_string, "add_root_to_linker<T: 'static");

    // eprintln!("{as_string}");

    proc_macro::TokenStream::from_str(&as_string).unwrap()
}

fn replace_namespace(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let as_string = stream.to_string();

    // Replace wasmtime:: package path with wasm_bridge::
    let regex = Regex::new("wasmtime[^:]*::").unwrap();
    let as_string = regex.replace_all(&as_string, "wasm_bridge::");

    eprintln!("REPLACED: {as_string}");

    proc_macro::TokenStream::from_str(&as_string).unwrap()
}
