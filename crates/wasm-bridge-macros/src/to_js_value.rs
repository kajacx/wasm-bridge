use proc_macro2::*;
use quote::{format_ident, quote, TokenStreamExt};
use syn::{DataEnum, DataStruct};

use crate::original::component::Flags;

pub fn to_js_value_struct(name: Ident, data: DataStruct) -> TokenStream {
    let mut impl_block = TokenStream::new();

    let field_count = data.fields.len();

    for (i, field) in data.fields.into_iter().enumerate() {
        let field_name = field.ident;

        // let field_name_str = quote!(#field_name).to_string();
        // let field_name_converted = field_name_str.to_lower_camel_case();

        let i = i as u32;
        let tokens = quote!(
            value.set(#i, self.#field_name.to_js_value());
        );

        impl_block.append_all(tokens);
    }

    let field_count = field_count as u32;

    quote! {
        impl wasm_bridge::ToJsValue for #name {
            type ReturnAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn to_js_value(&self) -> wasm_bridge::wasm_bindgen::JsValue {
                let value  = wasm_bridge::js_sys::Array::new_with_length(#field_count);

                #impl_block

                let value: wasm_bridge::wasm_bindgen::JsValue = value.into();

                value
            }

            fn into_return_abi(self) -> Result<Self::ReturnAbi, wasm_bridge::wasm_bindgen::JsValue> {
                Ok(self.to_js_value())
            }
        }

    }
}

pub(crate) fn to_js_value_flags(name: Ident, data: &Flags) -> TokenStream {
    let fields = data.flags.iter().enumerate().map(|(i, field)| {
        let field_name = format_ident!("{}", &field.name);

        // let field_name_str = quote!(#field_name).to_string();
        // let field_name_converted = field_name_str.to_lower_camel_case();

        let i = i as u32;
        quote!(
            value |= (self.#field_name as u32) << #i;
        )
    });

    quote! {
        impl wasm_bridge::ToJsValue for #name {
            type ReturnAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn to_js_value(&self) -> wasm_bridge::wasm_bindgen::JsValue {
                let mut value = 0;

                #(#fields)*

                let value: wasm_bridge::wasm_bindgen::JsValue = value.into();

                value
            }

            fn into_return_abi(self) -> Result<Self::ReturnAbi, wasm_bridge::wasm_bindgen::JsValue> {
                Ok(self.to_js_value())
            }
        }

    }
}

pub fn to_js_value_enum(name: Ident, data: DataEnum) -> TokenStream {
    let mut impl_block = TokenStream::new();

    for (i, variant) in data.variants.into_iter().enumerate() {
        let variant_name = variant.ident;
        // let variant_name_str = quote!(#variant_name).to_string();
        // let variant_name_converted = variant_name_str.to_kebab_case();

        let i = i as u8;
        let return_value = quote!(
            Self::#variant_name => {
                #i.into()
            },
        );

        impl_block.append_all(return_value);
    }

    quote! {
        impl wasm_bridge::ToJsValue for #name {
            type ReturnAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn to_js_value(&self) -> wasm_bridge::wasm_bindgen::JsValue {
                match self {
                    #impl_block
                }
            }

            fn into_return_abi(self) -> Result<Self::ReturnAbi, wasm_bridge::wasm_bindgen::JsValue> {
                Ok(self.to_js_value())
            }
        }
    }
}

pub fn to_js_value_variant(name: Ident, data: DataEnum) -> TokenStream {
    let init_block = quote! {

        // [ tag, val ]
        let result = wasm_bridge::js_sys::Array::new_with_length(2);
    };

    let ret_block = quote! {
        let result: wasm_bridge::wasm_bindgen::JsValue = result.into();
        result
    };

    let variants: TokenStream = data
        .variants
        .into_iter()
        .enumerate()
        .flat_map(|(i, variant)| {
            let i = i as u8;
            let variant_name = variant.ident;

            let field = variant.fields.iter().next();
            if field.is_some() {
                quote!(
                    Self::#variant_name(value) => {
                        result.set(0, #i.into());
                        result.set(1, value.to_js_value());
                    },
                )
            } else {
                quote!(
                    Self::#variant_name => {
                        result.set(0, #i.into());
                    },
                )
            }
        })
        .collect();

    quote! {
        impl wasm_bridge::ToJsValue for #name {
            type ReturnAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn to_js_value(&self) -> wasm_bridge::wasm_bindgen::JsValue {
                #init_block
                match self {
                    #variants
                }

                #ret_block
            }

            fn into_return_abi(self) -> Result<Self::ReturnAbi, wasm_bridge::wasm_bindgen::JsValue> {
                Ok(self.to_js_value())
            }
        }
    }
}
