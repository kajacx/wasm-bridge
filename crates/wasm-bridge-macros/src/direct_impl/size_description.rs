use std::str::FromStr;

use proc_macro2::*;
use quote::{format_ident, quote};
use syn::{DataEnum, DataStruct};

pub fn size_description_struct(name: Ident, data: DataStruct) -> TokenStream {
    let field_count = data.fields.len();
    let field_count_token = num_to_token(field_count);

    let mut alignment_impl = TokenStream::new();
    for field in data.fields.iter() {
        let field_type = &field.ty;
        let line = quote!(let align = usize::max(align, <#field_type>::alignment()););
        alignment_impl.extend(line);
    }

    let mut num_args = TokenStream::new();
    for field in data.fields.iter() {
        let field_type = &field.ty;
        num_args.extend(quote!( + <#field_type>::num_args()));
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

    let name_impl = format_ident!("impl_size_description_{}", name);
    quote!(
      mod #name_impl {
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

            fn num_args() -> usize {
                0 #num_args
            }

            fn layout() -> Self::StructLayout {
                let align = Self::alignment();
                let start0 = 0;
                #layout_impl
                [start0, #layout_return]
            }
        }
      }
    )
}

pub fn size_description_enum(_name: Ident, _data: DataEnum) -> TokenStream {
    todo!()
}

pub fn size_description_variant(_name: Ident, _data: DataEnum) -> TokenStream {
    todo!()
}

fn num_to_token(num: usize) -> TokenStream {
    TokenStream::from_str(&num.to_string()).unwrap()
}
