use syn::{parse_macro_input, DeriveInput, Error};

mod bindgen;
pub(crate) mod component;

pub fn lift(input: &DeriveInput) -> proc_macro::TokenStream {
    component::expand(&component::LiftExpander, input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

pub fn lower(input: &DeriveInput) -> proc_macro::TokenStream {
    component::expand(&component::LowerExpander, input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

pub fn component_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    component::expand(
        &component::ComponentTypeExpander,
        &parse_macro_input!(input as DeriveInput),
    )
    .unwrap_or_else(Error::into_compile_error)
    .into()
}

pub fn flags(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    component::expand_flags(&parse_macro_input!(input as component::Flags))
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

pub fn bindgen(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bindgen::expand(&parse_macro_input!(input as bindgen::Config))
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
