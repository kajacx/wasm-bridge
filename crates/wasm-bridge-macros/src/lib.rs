use std::str::FromStr;

use heck::ToLowerCamelCase;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
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

    // Remove the "ComponentType" trait, it's about memory and type safety, we don't need to care about it as much
    let regex = Regex::new("#\\[derive[^C]*ComponentType\\s*\\)\\s*\\]").unwrap();
    let as_string = regex.replace_all(&as_string, "");

    let regex = Regex::new("const _ : \\(\\) =[^}]*ComponentType[^}]*\\}\\s*;").unwrap();
    let as_string = regex.replace_all(&as_string, "");

    // Remove the "component" macro, we don't care about it either
    let regex = Regex::new("#\\[component\\([^)]*\\)\\]").unwrap();
    let as_string = regex.replace_all(&as_string, "");

    // Replace the "Lift" trait with "FromJsValue"
    let regex = Regex::new("#\\[derive\\([^)]*Lift\\)\\]").unwrap();
    let as_string = regex.replace_all(&as_string, "#[derive(wasm_bridge::component::FromJsValue)]");

    // Replace the "Lower" trait with "ToJsValue"
    let regex = Regex::new("#\\[derive\\([^)]*Lower\\)\\]").unwrap();
    let as_string = regex.replace_all(&as_string, "#[derive(wasm_bridge::component::ToJsValue)]");

    // eprintln!("{as_string}");

    proc_macro::TokenStream::from_str(&as_string).unwrap()
}

#[proc_macro_derive(FromJsValue)]
pub fn from_js_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput = syn::parse(input.clone()).unwrap();

    let struct_name = derive_input.ident;
    let data = match derive_input.data {
        syn::Data::Struct(struct_data) => struct_data,
        _ => todo!(),
    };

    let mut impl_block = String::new();
    let mut fields_constructor = String::new();

    for field in data.fields {
        let field_type = field.ty;
        let field_name = field.ident;

        let field_name_str = quote::quote!(#field_name).to_string();
        let field_name_converted = field_name_str.to_lower_camel_case();

        let tokens = quote::quote!(
            let js_field = wasm_bridge::js_sys::Reflect::get(value, &#field_name_converted.into())?;
            let #field_name = #field_type::from_js_value(&js_field)?;
        );

        impl_block.push_str(&tokens.to_string());

        fields_constructor.push_str(&format!("{}, ", field_name.unwrap().to_string()));
    }

    let impl_block = TokenStream::from_str(&impl_block).unwrap();
    let fields_constructor = TokenStream::from_str(&fields_constructor).unwrap();

    let tokens = quote::quote! {
        impl wasm_bridge::FromJsValue for #struct_name {
            type WasmAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn from_js_value(value: &wasm_bridge::wasm_bindgen::JsValue) -> wasm_bridge::Result<Self> {
                #impl_block

                Ok(Self { #fields_constructor })
            }

            fn from_wasm_abi(abi: Self::WasmAbi) -> wasm_bridge::Result<Self> {
                Self::from_js_value(&abi)
            }
        }
    };

    proc_macro::TokenStream::from_str(&tokens.to_string()).unwrap()
}

#[proc_macro_derive(ToJsValue)]
pub fn to_js_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: syn::DeriveInput = syn::parse(input.clone()).unwrap();

    let struct_name = derive_input.ident;
    let data = match derive_input.data {
        syn::Data::Struct(struct_data) => struct_data,
        _ => todo!(),
    };

    let mut impl_block = String::new();

    for field in data.fields {
        // let field_type = field.ty;
        let field_name = field.ident;

        let field_name_str = quote::quote!(#field_name).to_string();
        let field_name_converted = field_name_str.to_lower_camel_case();

        let tokens = quote::quote!(
            // let js_field = wasm_bridge::js_sys::Reflect::get(value, &#field_name_converted.into())?;
            // let #field_name = #field_type::from_js_value(&js_field)?;
            wasm_bridge::js_sys::Reflect::set(
                &value,
                &#field_name_converted.into(),
                &self.#field_name.to_js_value(),
            ).expect("value is object");
        );

        impl_block.push_str(&tokens.to_string());
    }

    let impl_block = TokenStream::from_str(&impl_block).unwrap();

    let tokens = quote::quote! {
        impl wasm_bridge::ToJsValue for #struct_name {
            type ReturnAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn to_js_value(&self) -> wasm_bridge::wasm_bindgen::JsValue {
                let value  = wasm_bridge::js_sys::Object::new();
                let value: wasm_bridge::wasm_bindgen::JsValue = value.into();

                #impl_block

                value
            }

            fn to_return_abi(&self) -> Self::ReturnAbi {
                self.to_js_value()
            }
        }
    };

    eprintln!("FINAL IMPL: {}", tokens.to_string());

    proc_macro::TokenStream::from_str(&tokens.to_string()).unwrap()
}

fn replace_namespace(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let as_string = stream.to_string();

    // Replace wasmtime:: package path with wasm_bridge::
    let regex = Regex::new("wasmtime[^:]*::").unwrap();
    let as_string = regex.replace_all(&as_string, "wasm_bridge::");

    proc_macro::TokenStream::from_str(&as_string).unwrap()
}
