use proc_macro2::*;
use quote::{format_ident, quote, TokenStreamExt};
use syn::{DataEnum, DataStruct};

use crate::original::component::Flags;

pub fn from_js_value_struct(name: Ident, data: DataStruct) -> TokenStream {
    let mut impl_block = TokenStream::new();
    let mut fields_constructor = TokenStream::new();

    for (i, field) in data.fields.into_iter().enumerate() {
        let field_type = field.ty;
        let field_name = field.ident;

        let i = i as u32;

        let tokens = quote!(
            let js_field = value.get(#i);
            let #field_name = <#field_type>::from_js_value(&js_field)?;
        );

        impl_block.append_all(tokens);

        fields_constructor.append_all(quote!(#field_name, ));
    }

    quote! {
        impl wasm_bridge::FromJsValue for #name {
            type WasmAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn from_js_value(value: &wasm_bridge::wasm_bindgen::JsValue) -> wasm_bridge::Result<Self> {
                use wasm_bridge::wasm_bindgen::JsCast;

                let value: &wasm_bridge::js_sys::Array = value.dyn_ref().unwrap();

                #impl_block

                Ok(Self { #fields_constructor })
            }

            fn from_wasm_abi(abi: Self::WasmAbi) -> wasm_bridge::Result<Self> {
                Self::from_js_value(&abi)
            }
        }
    }
}

pub(crate) fn from_js_value_flags(name: Ident, flags: &Flags) -> TokenStream {
    let fields = flags.flags.iter().enumerate().map(|(i, field)| {
        let field_name = format_ident!("{}", field.name);

        let i = i as u32;

        quote!(
            #field_name: value & (1 << #i) != 0,
        )
    });

    quote! {
        impl wasm_bridge::FromJsValue for #name {
            type WasmAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn from_js_value(value: &wasm_bridge::wasm_bindgen::JsValue) -> wasm_bridge::Result<Self> {
                use wasm_bridge::wasm_bindgen::JsCast;

                let value = u32::from_js_value(value)?;

                Ok( Self {
                    #(#fields)*
                } )
            }

            fn from_wasm_abi(abi: Self::WasmAbi) -> wasm_bridge::Result<Self> {
                Self::from_js_value(&abi)
            }
        }
    }
}
pub fn from_js_value_enum(name: Ident, data: DataEnum) -> TokenStream {
    let tokens = data.variants.into_iter().enumerate().map(|(i, variant)| {
        let variant_name = variant.ident;

        let i = i as u8;
        quote!(
            #i => {
                return Ok(Self::#variant_name)
            },
        )
    });

    quote! {
        impl wasm_bridge::FromJsValue for #name {
            type WasmAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn from_js_value(value: &wasm_bridge::wasm_bindgen::JsValue) -> wasm_bridge::Result<Self> {
                let tag = u8::from_js_value(value)?;
                match tag {
                    #(#tokens)*
                    _ => {
                        Err(wasm_bridge::helpers::map_js_error("Unknown enum tag")(value))
                    }
                }

            }

            fn from_wasm_abi(abi: Self::WasmAbi) -> wasm_bridge::Result<Self> {
                Self::from_js_value(&abi)
            }
        }
    }
}

pub fn from_js_value_variant(name: Ident, data: DataEnum) -> TokenStream {
    let tokens = data.variants.into_iter().enumerate().map(|(i, variant)| {
        let variant_name = variant.ident;

        let i = i as u8;

        let field = variant.fields.iter().next();

        let return_value = match field {
            Some(field) => {
                let field_type = &field.ty;
                quote!( Self::#variant_name(<#field_type>::from_js_value(&val)?) )
            }
            None => quote!( Self::#variant_name ),
        };

        quote!(
            #i => {
                Ok(#return_value)
            },
        )
    });

    quote! {
        impl wasm_bridge::FromJsValue for #name {
            type WasmAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn from_js_value(value: &wasm_bridge::wasm_bindgen::JsValue) -> wasm_bridge::Result<Self> {
                use wasm_bridge::wasm_bindgen::JsCast;
                let value: &wasm_bridge::js_sys::Array = value.dyn_ref().expect("variant is array");

                let tag = u8::from_js_value(&value.get(0))?;
                let val = value.get(1);

                match tag {
                    #(#tokens)*
                    _ => {
                        Err(wasm_bridge::helpers::map_js_error("Unknown variant tag")(value))
                    }
                }
            }

            fn from_wasm_abi(abi: Self::WasmAbi) -> wasm_bridge::Result<Self> {
                Self::from_js_value(&abi)
            }
        }
    }
}
