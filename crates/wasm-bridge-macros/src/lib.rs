use std::{ops::Deref, str::FromStr};

use original::{Style, VariantStyle};
use quote::ToTokens;
use regex::{Captures, Regex};
use syn::{Attribute, ImplItem, ItemImpl};

mod direct_impl;
mod original;

#[derive(Debug, Clone, PartialEq, Eq)]
enum CompilationTarget {
    Sys,
    Js,
}

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
pub fn flags_sys(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    replace_namespace(original::flags(input, CompilationTarget::Sys))
}

#[proc_macro]
pub fn flags_js(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    replace_namespace(original::flags(input, CompilationTarget::Js))
}

fn bindgen(input: proc_macro::TokenStream) -> String {
    let as_string = replace_namespace_str(original::bindgen(input));

    // Add PartialEq derive, so that testing isn't so miserably painful
    let regex = Regex::new("derive\\(([^\\)]*Clone[^\\)]*)\\)").unwrap();
    let as_string = regex.replace_all(&as_string, |caps: &Captures| {
        if caps[0].contains("PartialEq") {
            caps[0].to_string()
        } else {
            format!("derive({}, PartialEq)", &caps[1])
        }
    });

    as_string.to_string()
}

#[proc_macro]
pub fn bindgen_sys(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let as_string = bindgen(input);

    let as_string = add_safe_instantiation(&as_string);

    // eprintln!("bindgen SYS IMPL: {}", as_string.deref());
    proc_macro::TokenStream::from_str(&as_string).unwrap()
}

#[proc_macro]
pub fn bindgen_js(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let as_string = bindgen(input);

    // Clone exported function
    let regex = Regex::new("\\*\\s*__exports\\.typed_func([^?]*)\\?\\.func\\(\\)").unwrap();
    let as_string = regex.replace_all(&as_string, "__exports.typed_func$1?.func().clone()");

    // Clone "inner" function
    let regex = Regex::new("new_unchecked\\(self\\.([^)]*)\\)").unwrap();
    let as_string = regex.replace_all(&as_string, "new_unchecked(self.$1.clone())");

    let regex = Regex::new("add_to_linker\\s*<\\s*T").unwrap();
    let as_string = regex.replace_all(&as_string, "add_to_linker<T: 'static");

    let regex = Regex::new("add_root_to_linker\\s*<\\s*T").unwrap();
    let as_string = regex.replace_all(&as_string, "add_root_to_linker<T: 'static");

    // Remove the "ComponentType" trait, it's about memory and type safety, we don't need to care about it as much
    let regex = Regex::new("#\\[derive[^C]*ComponentType\\s*\\)\\s*\\]").unwrap();
    let as_string = regex.replace_all(&as_string, "");

    let regex =
        Regex::new("const\\s*_\\s*:\\s*\\(\\)\\s*=[^}]*ComponentType[^}]*\\}\\s*;").unwrap();
    let as_string = regex.replace_all(&as_string, "");

    // Replace the "Lift" trait with our Lift trait and SizeDescription
    let regex = Regex::new("#\\[derive\\([^)]*Lift\\)\\]").unwrap();
    let as_string = regex.replace_all(&as_string, "#[derive(wasm_bridge::component::SizeDescription)]\n#[derive(wasm_bridge::component::LiftJs)]");

    // Replace the "Lower" trait with out Lower trait
    let regex = Regex::new("#\\[derive\\([^)]*Lower\\)\\]").unwrap();
    let as_string = regex.replace_all(&as_string, "#[derive(wasm_bridge::component::LowerJs)]");

    let as_string = add_safe_instantiation(&as_string);

    // eprintln!("bindgen JS IMPL: {}", as_string.deref());
    proc_macro::TokenStream::from_str(&as_string).unwrap()
}

fn add_safe_instantiation(as_string: &str) -> impl Deref<Target = str> + '_ {
    let regex = Regex::new("pub\\s+fn\\s+instantiate\\s*<([^{]*)\\{").unwrap();

    regex.replace_all(as_string, r#"
    pub async fn instantiate_safe<T>(
        mut store: impl wasm_bridge::AsContextMut<Data = T>,
        component: &wasm_bridge::component::Component,
        linker: &wasm_bridge::component::Linker<T>,
    ) -> wasm_bridge::Result<(Self, wasm_bridge::component::Instance)> {
        let instance = linker.instantiate_safe(&mut store, component).await?;
        Ok((Self::new(store, &instance)?, instance))
    }
    
    #[deprecated(
        since = "0.4.0",
        note = "Instantiating a component synchronously can panic on the web, please use `instantiate_safe` instead."
    )]
    pub fn instantiate< $1 {
        #[allow(deprecated)]
        "#)
}

#[proc_macro_derive(SizeDescription, attributes(component))]
pub fn derive_size_description(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput = syn::parse(input).unwrap();

    let name = derive_input.ident;
    let struct_style = style_from_attributes(&derive_input.attrs);

    let tokens = match derive_input.data {
        syn::Data::Struct(data) => direct_impl::size_description_struct(name, data),
        syn::Data::Enum(data) => match struct_style.expect("cannot find attribute style") {
            Style::Record => unreachable!("enum is not a record"),
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
        syn::Data::Enum(data) => match struct_style.expect("cannot find attribute style") {
            Style::Record => unreachable!("enum is not a record"),
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
        syn::Data::Enum(data) => match struct_style.expect("cannot find attribute style") {
            Style::Record => unreachable!("enum is not a record"),
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
    let mut item_impl: ItemImpl = syn::parse(input).unwrap();
    for item in item_impl.items.iter_mut() {
        if let ImplItem::Fn(method) = item {
            method.sig.asyncness = None;
        }
    }
    item_impl.into_token_stream().into()
}

fn replace_namespace_str(stream: proc_macro::TokenStream) -> String {
    let as_string = stream.to_string();

    // Replace wasmtime:: package path with wasm_bridge::
    let regex = Regex::new("wasmtime[^:]*::").unwrap();
    let as_string = regex.replace_all(&as_string, "wasm_bridge::");

    as_string.to_string()
}

fn replace_namespace(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let as_string = replace_namespace_str(stream);

    proc_macro::TokenStream::from_str(&as_string).unwrap()
}

fn style_from_attributes(attributes: &[Attribute]) -> Option<Style> {
    attributes
        .iter()
        .find(|attr| attr.path().is_ident("component"))
        .map(|attr| {
            attr.parse_args()
                .expect("Failed to parse Style from Attribute")
        })
}

#[proc_macro]
pub fn size_description_tuple(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    direct_impl::size_description_tuple(tokens)
}
