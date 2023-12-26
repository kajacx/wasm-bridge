use std::str::FromStr;

use proc_macro2::*;
use quote::{format_ident, quote};
use syn::{DataEnum, DataStruct};

pub fn lift_struct(name: Ident, data: DataStruct) -> TokenStream {
    let fields = data.fields;
    let field_count = fields.len();

    let from_js_return = if field_count == 1 {
        let field_type = &fields.iter().next().unwrap().ty;
        quote!(<#field_type>::from_js_return(value, memory))
    } else {
        quote!(Self::from_js_ptr_return(value, memory))
    };

    let mut from_js_args = TokenStream::new();
    for field in fields.iter() {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let line = quote!(#field_name: <#field_type>::from_js_args(args, memory)?,);
        from_js_args.extend(line);
    }

    let mut read_from_impl = TokenStream::new();
    for (i, field) in fields.iter().enumerate() {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let start = num_to_token(i * 2);
        let end = num_to_token(i * 2 + 1);
        let line = quote!(#field_name: <#field_type>::read_from(&slice[layout[#start]..layout[#end]], memory)?,);
        read_from_impl.extend(line);
    }

    let name_impl = format_ident!("impl_lift_{}", name);
    quote!(
      mod #name_impl {
        use wasm_bridge::direct_bytes::*;
        use wasm_bridge::FromJsValue;
        use super::*;

        impl wasm_bridge::direct_bytes::Lift for #name {
            fn from_js_return<M: wasm_bridge::direct_bytes::ReadableMemory>(value: &wasm_bridge::wasm_bindgen::JsValue, memory: &M) -> wasm_bridge::Result<Self> {
                #from_js_return
            }

            fn from_js_args<M: wasm_bridge::direct_bytes::ReadableMemory>(args: &mut wasm_bridge::direct_bytes::JsArgsReader, memory: &M) -> wasm_bridge::Result<Self> {
                Ok(Self { #from_js_args })
            }

            fn read_from<M: wasm_bridge::direct_bytes::ReadableMemory>(slice: &[u8], memory: &M) -> wasm_bridge::Result<Self> {
                let layout = Self::layout();
                Ok(Self { #read_from_impl })
            }
        }
      }
    )
}

pub fn lift_enum(name: Ident, data: DataEnum) -> TokenStream {
    let name_str = format_ident!("{}", name).to_string();
    let variants = data.variants;

    let mut from_js_return = TokenStream::new();
    for (i, variant) in variants.iter().enumerate() {
        let i_u8 = i as u8;
        let name = &variant.ident;
        let line = quote!(#i_u8 => Self::#name,);
        from_js_return.extend(line);
    }

    let name_impl = format_ident!("impl_lift_{}", name);
    quote!(
      mod #name_impl {
        use wasm_bridge::direct_bytes::*;
        use wasm_bridge::FromJsValue;
        use wasm_bridge::Context;
        use super::*;

        impl wasm_bridge::direct_bytes::Lift for #name {
            fn from_js_return<M: wasm_bridge::direct_bytes::ReadableMemory>(value: &wasm_bridge::wasm_bindgen::JsValue, memory: &M) -> wasm_bridge::Result<Self> {
                let tag = u8::from_js_value(value)?;
                Ok(match tag {
                    #from_js_return
                    other => wasm_bridge::bail!("Invalid tag {other} for enum {}", #name_str)
                })
            }

            fn from_js_args<M: wasm_bridge::direct_bytes::ReadableMemory>(args: &mut wasm_bridge::direct_bytes::JsArgsReader, memory: &M) -> wasm_bridge::Result<Self> {
                let value = args.next().context("Get enum tag")?;
                Self::from_js_return(&value, memory)
            }

            fn read_from<M: wasm_bridge::direct_bytes::ReadableMemory>(slice: &[u8], memory: &M) -> wasm_bridge::Result<Self> {
                let tag = slice[0];
                Ok(match tag {
                    #from_js_return
                    other => wasm_bridge::bail!("Invalid tag {other} for enum {}", #name_str)
                })
            }
        }
      }
    )
}

pub fn lift_variant(name: Ident, data: DataEnum) -> TokenStream {
    let name_str = format_ident!("{}", name).to_string();
    let variants = data.variants;
    let all_empty = variants.iter().all(|variant| variant.fields.len() == 0);

    let from_js_return = if all_empty {
        let match_arms: TokenStream = variants
            .iter()
            .enumerate()
            .map(|(i, variant)| {
                let tag = i as u8;
                let variant_name = &variant.ident;
                quote!(#tag => Self::#variant_name,)
            })
            .collect();

        quote!(
            let tag = u8::from_js_value(value)?;
            Ok(match tag {
                #match_arms
                other => wasm_bridge::bail!("Invalid tag {other} for variant {}", #name_str)
            })
        )
    } else {
        quote!(Self::from_js_ptr_return(value, memory))
    };

    let from_js_args: TokenStream = variants.iter().enumerate().map(|(i, variant)| {
        let tag = i as u8;
        let variant_name = &variant.ident;
        if let Some(field) = variant.fields.iter().next() {
            let field_type = &field.ty;
            quote!(#tag => (Self::#variant_name(<#field_type>::from_js_args(args, memory)?), <#field_type>::num_args()),)
        } else {
            quote!(#tag => (Self::#variant_name, 0),)
        }
    }).collect();

    let read_from: TokenStream = variants.iter().enumerate().map(|(i, variant)| {
        let tag = i as u8;
        let variant_name = &variant.ident;
        if let Some(field) = variant.fields.iter().next() {
            let field_type = &field.ty;
            quote!(#tag => Self::#variant_name(<#field_type>::read_from(&slice[(Self::ALIGNMENT)..(Self::ALIGNMENT + <#field_type>::FLAT_BYTE_SIZE)], memory)?),)
        } else {
            quote!(#tag => Self::#variant_name,)
        }
    }).collect();

    let name_impl = format_ident!("impl_lift_{}", name);
    quote!(
      mod #name_impl {
        use wasm_bridge::direct_bytes::*;
        use wasm_bridge::FromJsValue;
        use wasm_bridge::Context;
        use super::*;

        impl wasm_bridge::direct_bytes::Lift for #name {
            fn from_js_return<M: wasm_bridge::direct_bytes::ReadableMemory>(value: &wasm_bridge::wasm_bindgen::JsValue, memory: &M) -> wasm_bridge::Result<Self> {
                #from_js_return
            }

            fn from_js_args<M: wasm_bridge::direct_bytes::ReadableMemory>(args: &mut wasm_bridge::direct_bytes::JsArgsReader, memory: &M) -> wasm_bridge::Result<Self> {
                let tag = args.next().context("Get variant tag")?;
                let tag = u8::from_js_value(&tag)?;

                let (result, args_read) = match tag {
                    #from_js_args
                    other => wasm_bridge::bail!("Invalid tag {other} for variant {}", #name_str)
                };

                // Start from 1 to account for the initial variant tag
                for _ in 1..(Self::num_args() - args_read) {
                    args.next().context("Skipping unused result args")?;
                }
                Ok(result)
            }

            fn read_from<M: wasm_bridge::direct_bytes::ReadableMemory>(slice: &[u8], memory: &M) -> wasm_bridge::Result<Self> {
                let tag = slice[0];
                Ok(match tag {
                    #read_from
                    other => wasm_bridge::bail!("Invalid tag {other} for variant {}", #name_str)
                })
            }
        }
      }
    )
}

fn num_to_token(num: usize) -> TokenStream {
    TokenStream::from_str(&num.to_string()).unwrap()
}
