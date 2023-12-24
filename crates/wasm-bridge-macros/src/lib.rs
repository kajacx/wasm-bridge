use std::str::FromStr;

use original::{Style, VariantStyle};
use regex::Regex;
use syn::Attribute;

mod original;

#[cfg(not(feature = "direct-bytes"))]
mod js_impl;
#[cfg(not(feature = "direct-bytes"))]
mod direct_impl {
    pub use super::js_impl::from_js_value_enum as lift_enum;
    pub use super::js_impl::from_js_value_struct as lift_struct;
    pub use super::js_impl::from_js_value_variant as lift_variant;

    pub use super::js_impl::to_js_value_enum as lower_enum;
    pub use super::js_impl::to_js_value_struct as lower_struct;
    pub use super::js_impl::to_js_value_variant as lower_variant;

    pub use super::js_impl::size_description_enum;
    pub use super::js_impl::size_description_struct;
    pub use super::js_impl::size_description_variant;
}

#[cfg(feature = "direct-bytes")]
mod direct_impl;

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
    replace_namespace(original::bindgen(input))
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

    // Remove the "ComponentType" trait, it's about memory and type safety, we don't need to care about it as much
    let regex = Regex::new("#\\[derive[^C]*ComponentType\\s*\\)\\s*\\]").unwrap();
    let as_string = regex.replace_all(&as_string, "");

    let regex = Regex::new("const _ : \\(\\) =[^}]*ComponentType[^}]*\\}\\s*;").unwrap();
    let as_string = regex.replace_all(&as_string, "");

    // Replace the "Lift" trait with our Lift trait and SizeDescription
    let regex = Regex::new("#\\[derive\\([^)]*Lift\\)\\]").unwrap();
    let as_string = regex.replace_all(&as_string, "#[derive(wasm_bridge::component::SizeDescription)]\n#[derive(wasm_bridge::component::LiftJs)]");

    // Replace the "Lower" trait with out Lower trait
    let regex = Regex::new("#\\[derive\\([^)]*Lower\\)\\]").unwrap();
    let as_string = regex.replace_all(&as_string, "#[derive(wasm_bridge::component::LowerJs)]");

    // Remove asynchrony
    let as_string = if cfg!(feature = "async") {
        let regex = Regex::new("Box[^:]*::[^n]*new[^(]*\\([^a]*async[^m]*move").unwrap();
        let as_string = regex.replace_all(&as_string, "(");

        // TODO: this removes "await"s even in places where it isn't supposed to
        as_string.replace(".await", "")
    } else {
        as_string.to_string()
    };

    // eprintln!("#[cfg(test)]");
    // eprintln!("#[allow(warnings)]");
    // eprintln!("mod test {{");
    // eprintln!("  pub mod wasm_bridge {{");
    // eprintln!("    pub use crate::*;");
    // eprintln!("  }}");
    // eprintln!("  {as_string}");
    // eprintln!("}}");

    proc_macro::TokenStream::from_str(&as_string).unwrap()
}

#[proc_macro_derive(SizeDescription, attributes(component))]
pub fn derive_size_description(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput = syn::parse(input).unwrap();

    let name = derive_input.ident;
    let struct_style = style_from_attributes(&derive_input.attrs);

    let tokens = match derive_input.data {
        syn::Data::Struct(data) => direct_impl::size_description_struct(name, data),
        syn::Data::Enum(data) => match struct_style.expect("TODO: better error message") {
            Style::Record => unreachable!("TODO: better error message"),
            Style::Variant(VariantStyle::Enum) => direct_impl::size_description_enum(name, data),
            Style::Variant(VariantStyle::Variant) => {
                direct_impl::size_description_variant(name, data)
            }
        },
        syn::Data::Union(_) => unimplemented!("Union type should not be generated by wit bindgen"),
    };

    // eprintln!("derive_size_description IMPL: {}", tokens);
    proc_macro::TokenStream::from(tokens)
}

#[proc_macro_derive(LiftJs, attributes(component))]
pub fn derive_lift(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput = syn::parse(input).unwrap();

    let name = derive_input.ident;
    let struct_style = style_from_attributes(&derive_input.attrs);

    let tokens = match derive_input.data {
        syn::Data::Struct(data) => direct_impl::lift_struct(name, data),
        syn::Data::Enum(data) => match struct_style.expect("TODO: better error message") {
            Style::Record => unreachable!("TODO: better error message"),
            Style::Variant(VariantStyle::Enum) => direct_impl::lift_enum(name, data),
            Style::Variant(VariantStyle::Variant) => direct_impl::lift_variant(name, data),
        },
        syn::Data::Union(_) => unimplemented!("Union type should not be generated by wit bindgen"),
    };

    // eprintln!("derive_lift IMPL: {}", tokens);
    proc_macro::TokenStream::from(tokens)
}

#[proc_macro_derive(LowerJs, attributes(component))]
pub fn derive_lower(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput = syn::parse(input).unwrap();

    let name = derive_input.ident;
    let struct_style = style_from_attributes(&derive_input.attrs);

    let tokens = match derive_input.data {
        syn::Data::Struct(data) => direct_impl::lower_struct(name, data),
        syn::Data::Enum(data) => match struct_style.expect("TODO: better error message") {
            Style::Record => unreachable!("TODO: better error message"),
            Style::Variant(VariantStyle::Enum) => direct_impl::lower_enum(name, data),
            Style::Variant(VariantStyle::Variant) => direct_impl::lower_variant(name, data),
        },
        syn::Data::Union(_) => unimplemented!("Union type should not be generated by wit bindgen"),
    };

    // eprintln!("derive_lower IMPL: {}", tokens);
    proc_macro::TokenStream::from(tokens)
}

#[proc_macro_attribute]
pub fn async_trait(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let as_string = input.to_string();

    // TODO: this is a really hacky way to do it
    let regex = Regex::new("async\\s*fn").unwrap();
    let as_string = regex.replace_all(&as_string, "fn");

    proc_macro::TokenStream::from_str(&as_string).unwrap()
}

fn replace_namespace(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let as_string = stream.to_string();

    // Replace wasmtime:: package path with wasm_bridge::
    let regex = Regex::new("wasmtime[^:]*::").unwrap();
    let as_string = regex.replace_all(&as_string, "wasm_bridge::");

    proc_macro::TokenStream::from_str(&as_string).unwrap()
}

fn style_from_attributes(attributes: &[Attribute]) -> Option<Style> {
    attributes
        .iter()
        .find(|attr| attr.path().is_ident("component"))
        .map(|attr| {
            // TODO: Better error message
            attr.parse_args()
                .expect("Attribute should be correct style")
        })
}
