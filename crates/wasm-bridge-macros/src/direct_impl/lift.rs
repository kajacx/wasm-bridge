use heck::{ToKebabCase, ToLowerCamelCase};
use proc_macro2::*;
use quote::{quote, TokenStreamExt};
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
        let start = i * 2;
        let end = i * 2 + 1;
        let line = quote!(<#field_type>::read_from(&slice[layout[#start]..layout[#end]], memory)?,);
        read_from_impl.extend(line);
    }

    let mut alignment_impl = TokenStream::new();
    for field in data.fields.iter() {
        let field_type = &field.ty;
        let line = quote!(let align = usize::max(align, <#field_type>::alignment()););
        alignment_impl.extend(line);
    }

    quote!(
        impl wasm_bridge::direct_bytes::SizeDescription for #name {
            type StructLayout = [usize; #field_count * 2 + 1]

            fn alignment() -> usize {
                let align = 0;
                #alignment_impl
                align
            }

            fn flat_byte_size() -> usize {
                self::layout()[#field_count*2]
            }

            fn layout() -> Self::StructLayout {
                // TODO
            }
        }

        impl wasm_bridge::direct_bytes::Lift for #name {
            fn from_js_return<M: wasm_bridge::direct_bytes::ReadableMemory>(val: &wasm_bridge::wasm_bindgen::JsValue, memory: &M) -> wasm_bridge::Result<Self> {
                #from_js_return
            }

            fn read_from<M: wasm_bridge::direct_bytes::ReadableMemory>(slice: &[u8], memory: &M) -> wasm_bridge::result::Result<Self> {
                let layout = Self::layout();
                Ok((#read_from_impl))
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
