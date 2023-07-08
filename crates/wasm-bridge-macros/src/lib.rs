use std::str::FromStr;

use regex::Regex;
use syn::{parse_macro_input, DeriveInput, Error};

mod bindgen;
mod component;

#[proc_macro_derive(Lift, attributes(component))]
pub fn lift(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    component::expand(
        &component::LiftExpander,
        &parse_macro_input!(input as DeriveInput),
    )
    .unwrap_or_else(Error::into_compile_error)
    .into()
}

#[proc_macro_derive(Lower, attributes(component))]
pub fn lower(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    component::expand(
        &component::LowerExpander,
        &parse_macro_input!(input as DeriveInput),
    )
    .unwrap_or_else(Error::into_compile_error)
    .into()
}

#[proc_macro_derive(ComponentType, attributes(component))]
pub fn component_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    component::expand(
        &component::ComponentTypeExpander,
        &parse_macro_input!(input as DeriveInput),
    )
    .unwrap_or_else(Error::into_compile_error)
    .into()
}

#[proc_macro]
pub fn flags(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    component::expand_flags(&parse_macro_input!(input as component::Flags))
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn bindgen(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let stream: proc_macro::TokenStream =
        bindgen::expand(&parse_macro_input!(input as bindgen::Config))
            .unwrap_or_else(Error::into_compile_error)
            .into();

    let as_string = stream.to_string();

    let regex = Regex::new("\\*\\s*__exports\\.typed_func([^?]*)\\?\\.func\\(\\)").unwrap();
    let as_string = regex.replace_all(&as_string, "__exports.typed_func$1?.func().clone()");

    let regex = Regex::new("new_unchecked\\(self\\.([^)]*)\\)").unwrap();
    let as_string = regex.replace_all(&as_string, "new_unchecked(self.$1.clone())");

    let regex = Regex::new("let host = get\\(caller\\.data_mut\\(\\)\\)\\s*;").unwrap();
    let as_string = regex.replace_all(&as_string, "let host = get(&mut caller);\n");

    proc_macro::TokenStream::from_str(&as_string).unwrap()
}
