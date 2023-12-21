use std::str::FromStr;

use heck::{ToKebabCase, ToLowerCamelCase};
use proc_macro2::*;
use quote::{format_ident, quote, TokenStreamExt};
use syn::{DataEnum, DataStruct};

pub fn lift_struct(name: Ident, data: DataStruct) -> TokenStream {
    let field_count = data.fields.len();
    let field_count_token = num_to_token(field_count);

    let mut alignment_impl = TokenStream::new();
    for field in data.fields.iter() {
        let field_type = &field.ty;
        let line = quote!(let align = usize::max(align, <#field_type>::alignment()););
        alignment_impl.extend(line);
    }

    let mut layout_impl = TokenStream::new();
    let mut layout_return = TokenStream::new();
    for (i, field) in data.fields.iter().enumerate() {
        let field_type = &field.ty;

        let start_i = format_ident!("start{i}");
        let end_i = format_ident!("end{i}");
        let start_next = format_ident!("start{}", i + 1);

        let line = quote!(let #end_i = #start_i + <#field_type>::flat_byte_size(););
        layout_impl.extend(line);

        let line =
            quote!(let #start_next = wasm_bridge::direct_bytes::next_multiple_of(#end_i, align););
        layout_impl.extend(line);

        let ret = quote!(#end_i, #start_next,);
        layout_return.extend(ret);
    }

    // TODO: what if field count is 0?

    let from_js_return = if field_count == 1 {
        let field_type = &data.fields.iter().next().unwrap().ty;
        quote!(<#field_type>::from_js_return(val, memory))
    } else {
        quote!(
            let addr = u32::from_js_value(val)? as usize;
            let len = Self::flat_byte_size();

            let data = memory.read_to_vec(addr, len);
            Self::read_from(&data, memory)
        )
    };

    let mut read_from_impl = TokenStream::new();
    for (i, field) in data.fields.iter().enumerate() {
        let field_type = &field.ty;
        let field_name = &field.ident;
        let start = num_to_token(i * 2);
        let end = num_to_token(i * 2 + 1);
        let line = quote!(#field_name: <#field_type>::read_from(&slice[layout[#start]..layout[#end]], memory)?,);
        read_from_impl.extend(line);
    }

    quote!(
      mod lift_impls {
        use wasm_bridge::direct_bytes::*;
        use wasm_bridge::FromJsValue;
        use super::*;

        impl wasm_bridge::direct_bytes::SizeDescription for #name {
            type StructLayout = [usize; #field_count_token * 2 + 1];

            fn alignment() -> usize {
                let align = 0;
                #alignment_impl
                align
            }

            fn flat_byte_size() -> usize {
                Self::layout()[#field_count_token*2]
            }

            fn layout() -> Self::StructLayout {
                let align = Self::alignment();
                let start0 = 0;
                #layout_impl
                [start0, #layout_return]
            }
        }

        impl wasm_bridge::direct_bytes::Lift for #name {
            fn from_js_return<M: wasm_bridge::direct_bytes::ReadableMemory>(val: &wasm_bridge::wasm_bindgen::JsValue, memory: &M) -> wasm_bridge::Result<Self> {
                #from_js_return
            }

            fn read_from<M: wasm_bridge::direct_bytes::ReadableMemory>(slice: &[u8], memory: &M) -> wasm_bridge::Result<Self> {
                let layout = Self::layout();
                Ok(Self {#read_from_impl})
            }
        }
      }
    )
}

pub fn lift_enum(name: Ident, data: DataEnum) -> TokenStream {
    todo!()
}

pub fn lift_variant(name: Ident, data: DataEnum) -> TokenStream {
    todo!()
}

fn num_to_token(num: usize) -> TokenStream {
    TokenStream::from_str(&num.to_string()).unwrap()
}
