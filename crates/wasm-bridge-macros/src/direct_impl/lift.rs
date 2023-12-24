use std::str::FromStr;

use proc_macro2::*;
use quote::{format_ident, quote};
use syn::{DataEnum, DataStruct};

pub fn lift_struct(name: Ident, data: DataStruct) -> TokenStream {
    let field_count = data.fields.len();

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

            fn read_from<M: wasm_bridge::direct_bytes::ReadableMemory>(slice: &[u8], memory: &M) -> wasm_bridge::Result<Self> {
                let layout = Self::layout();
                Ok(Self {#read_from_impl})
            }
        }
      }
    )
}

pub fn lift_enum(_name: Ident, _data: DataEnum) -> TokenStream {
    todo!()
}

pub fn lift_variant(_name: Ident, _data: DataEnum) -> TokenStream {
    todo!()
}

fn num_to_token(num: usize) -> TokenStream {
    TokenStream::from_str(&num.to_string()).unwrap()
}
