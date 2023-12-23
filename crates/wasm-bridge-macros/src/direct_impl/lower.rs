use std::str::FromStr;

use proc_macro2::*;
use quote::{format_ident, quote};
use syn::{DataEnum, DataStruct};

pub fn lower_struct(name: Ident, data: DataStruct) -> TokenStream {


    let mut to_abi_impl = TokenStream::new();
    for field in data.fields.iter() {
        let field_name = &field.ident;
        let line = quote!(self.#field_name.to_js_args(args, memory)?;);
        to_abi_impl.extend(line);
    }

    let mut write_to_impl = TokenStream::new();
    for (i, field) in data.fields.iter().enumerate() {
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
        use super::*;

        impl wasm_bridge::direct_bytes::Lower for #name {
            fn to_js_args<M: wasm_bridge::direct_bytes::WriteableMemory>(&self, args: &mut Vec<wasm_bridge::wasm_bindgen::JsValue>, memory: &M) -> wasm_bridge::Result<()> {
                #to_abi_impl
                Ok(())
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

pub fn lower_enum(_name: Ident, _data: DataEnum) -> TokenStream {
    todo!()
}

pub fn lower_variant(_name: Ident, _data: DataEnum) -> TokenStream {
    todo!()
}

fn num_to_token(num: usize) -> TokenStream {
    TokenStream::from_str(&num.to_string()).unwrap()
}
