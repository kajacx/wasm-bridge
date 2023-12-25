use std::str::FromStr;

use proc_macro2::*;
use quote::{format_ident, quote};
use syn::{DataEnum, DataStruct};

pub fn lower_struct(name: Ident, data: DataStruct) -> TokenStream {
    let fields = data.fields;

    let mut to_abi_impl = TokenStream::new();
    for field in fields.iter() {
        let field_name = &field.ident;
        let line = quote!(self.#field_name.to_js_args(args, memory)?;);
        to_abi_impl.extend(line);
    }

    let mut write_to_impl = TokenStream::new();
    for (i, field) in fields.iter().enumerate() {
        let field_name = &field.ident;

        let end = num_to_token(i * 2 + 1);
        let start_next = num_to_token(i * 2 + 2);

        let line = quote!(self.#field_name.write_to(buffer, memory)?;);
        write_to_impl.extend(line);

        let line = quote!(buffer.skip(layout[#start_next] - layout[#end]););
        write_to_impl.extend(line);
    }

    let name_impl = format_ident!("impl_lower_{}", name);
    quote!(
      mod #name_impl {
        use wasm_bridge::direct_bytes::*;
        use wasm_bridge::ToJsValue;
        use super::*;

        impl wasm_bridge::direct_bytes::Lower for #name {
            fn to_js_args<M: wasm_bridge::direct_bytes::WriteableMemory>(&self, args: &mut Vec<wasm_bridge::wasm_bindgen::JsValue>, memory: &M) -> wasm_bridge::Result<()> {
                #to_abi_impl
                Ok(())
            }

            fn to_js_return<M: WriteableMemory>(&self, memory: &M) -> Result<wasm_bridge::wasm_bindgen::JsValue> {
                // FIXME: this surely doesn't work for 1-field structs?
                let mut buffer = memory.allocate(Self::alignment(), Self::flat_byte_size())?;
                self.write_to(&mut buffer, memory)?;

                let addr = memory.flush(buffer) as u32;
                Ok(addr.to_js_value())
            }

            fn write_to<M: wasm_bridge::direct_bytes::WriteableMemory>(&self, buffer: &mut wasm_bridge::direct_bytes::ByteBuffer, memory: &M) -> wasm_bridge::Result<()> {
                let layout = Self::layout();
                #write_to_impl
                Ok(())
            }
        }
      }
    )
}

pub fn lower_enum(name: Ident, data: DataEnum) -> TokenStream {
    let variants = data.variants;

    let mut match_arms = TokenStream::new();
    for (i, variant) in variants.iter().enumerate() {
        let i_u8 = i as u8;
        let name = &variant.ident;
        let line = quote!(Self::#name => #i_u8,);
        match_arms.extend(line);
    }

    let name_impl = format_ident!("impl_lower_{}", name);
    quote!(
      mod #name_impl {
        use wasm_bridge::direct_bytes::*;
        use wasm_bridge::ToJsValue;
        use super::*;

        impl wasm_bridge::direct_bytes::Lower for #name {
            fn to_js_args<M: wasm_bridge::direct_bytes::WriteableMemory>(&self, args: &mut Vec<wasm_bridge::wasm_bindgen::JsValue>, memory: &M) -> wasm_bridge::Result<()> {
                args.push(Self::to_js_return(memory)?);
                Ok(())
            }

            fn to_js_return<M: WriteableMemory>(&self, _memory: &M) -> Result<wasm_bridge::wasm_bindgen::JsValue> {
                let value = match self {
                    #match_arms
                };
                Ok(value.to_js_value())
            }

            fn write_to<M: wasm_bridge::direct_bytes::WriteableMemory>(&self, buffer: &mut wasm_bridge::direct_bytes::ByteBuffer, memory: &M) -> wasm_bridge::Result<()> {
                let value = match self {
                    #match_arms
                };
                value.write_to(&mut buffer, memory);
                Ok(())
            }
        }
      }
    )
}

pub fn lower_variant(name: Ident, data: DataEnum) -> TokenStream {
    let variants = data.variants;
    let all_empty = variants.iter().all(|variant| variant.fields.len() == 0);

    let match_arms = variants
        .iter()
        .enumerate()
        .map(|(i, variant)| {
            let tag = i as u8;
            let variant_name = &variant.ident;
            if let Some(field) = variant.fields.iter().next() {
                let field_type = &field.ty;
                quote!(
                    Self::#variant_name(value) => {
                        args.push(#tag.to_js_value());
                        value.to_js_args(args, memory)?;
                        <#field_type>::num_args()
                    }
                )
            } else {
                quote!(
                    Self::#variant_name() => {
                        args.push(#tag.to_js_value());
                        0
                    }
                )
            }
        })
        .collect::<TokenStream>();

    let to_js_args = quote!(
        let args_written = match self {
            #match_arms
        };

        // Start from 1 to account for the initial variant tag
        for _ in 1..(Self::num_args() - args_written) {
            args.push(JsValue::UNDEFINED);
        }
        Ok(())
    );

    let to_js_return = if all_empty {
        let match_arms = variants
            .iter()
            .enumerate()
            .map(|(i, variant)| {
                let tag = i as u8;
                let variant_name = &variant.ident;
                quote!(
                    Self::#variant_name() => {
                       #tag.to_js_value();
                    }
                )
            })
            .collect::<TokenStream>();

        quote!(
            let tag = args.next().context("Get variant tag")?;
            match self {
                #match_arms
            }
        )
    } else {
        quote!(
            let mut buffer = memory.allocate(Self::alignment(), Self::flat_byte_size())?;
            self.write_to(&mut buffer, memory)?;

            let addr = memory.flush(buffer) as u32;
            Ok(addr.to_js_value())
        )
    };

    let write_to = variants
        .iter()
        .enumerate()
        .map(|(i, variant)| {
            let i_u8 = i as u8;
            let variant_name = &variant.ident;
            if let Some(field) = variant.fields.iter().next() {
                let field_type = &field.ty;
                quote!(Self::#variant_name(value) => {
                    buffer.write(&#i_u8, memory)?;
                    buffer.skip(Self::alignment() - 1);

                    buffer.write(value, memory)?;
                    <#field_type>::flat_byte_size();
                },)
            } else {
                quote!(Self::#variant_name => {
                    buffer.write(&#i_u8, memory)?;
                    buffer.skip(Self::alignment() - 1);

                    0
                },)
            }
        })
        .collect::<TokenStream>();

    let name_impl = format_ident!("impl_lower_{}", name);
    quote!(
      mod #name_impl {
        use wasm_bridge::direct_bytes::*;
        use wasm_bridge::ToJsValue;
        use super::*;

        impl wasm_bridge::direct_bytes::Lower for #name {
            fn to_js_args<M: wasm_bridge::direct_bytes::WriteableMemory>(&self, args: &mut Vec<wasm_bridge::wasm_bindgen::JsValue>, memory: &M) -> wasm_bridge::Result<()> {
                #to_js_args
            }

            fn to_js_return<M: WriteableMemory>(&self, _memory: &M) -> Result<wasm_bridge::wasm_bindgen::JsValue> {
                #to_js_return
            }

            fn write_to<M: wasm_bridge::direct_bytes::WriteableMemory>(&self, buffer: &mut wasm_bridge::direct_bytes::ByteBuffer, memory: &M) -> wasm_bridge::Result<()> {
                let bytes_written = match self {
                    #write_to
                };

                // Variant tag takes 1 whole alignment
                buffer.skip(Self::flat_byte_size() - bytes_written - Self::alignment());
                Ok(())
            }
        }
      }
    )
}

fn num_to_token(num: usize) -> TokenStream {
    TokenStream::from_str(&num.to_string()).unwrap()
}
