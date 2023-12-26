use std::str::FromStr;

use proc_macro2::*;
use quote::{format_ident, quote};
use syn::{DataEnum, DataStruct};

pub fn size_description_struct(name: Ident, data: DataStruct) -> TokenStream {
    let fields = data.fields;
    let field_count = fields.len();
    let field_count_token = num_to_token(field_count);

    let mut alignment = quote!(1usize);
    for field in fields.iter() {
        let field_type = &field.ty;
        alignment = quote!(wasm_bridge::usize_max(#alignment, <#field_type>::ALIGNMENT));
    }

    let mut byte_size = quote!(0usize);
    let mut fields_peek = fields.iter().peekable();
    while let Some(field) = fields_peek.next() {
        let field_type = &field.ty;
        if let Some(next) = fields_peek.peek() {
            let next_type = &next.ty;
            byte_size = quote!(wasm_bridge::next_multiple_of(#byte_size + <#field_type>::BYTE_SIZE, <#next_type>::ALIGNMENT));
        } else {
            byte_size = quote!(wasm_bridge::next_multiple_of(#byte_size + <#field_type>::BYTE_SIZE, Self::ALIGNMENT));
        }
    }

    let mut num_args = quote!(0);
    for field in fields.iter() {
        let field_type = &field.ty;
        num_args.extend(quote!( + <#field_type>::NUM_ARGS));
    }

    let mut layout_impl = TokenStream::new();
    let mut layout_return = TokenStream::new();
    for (i, field) in fields.iter().enumerate() {
        let field_type = &field.ty;

        let start_i = format_ident!("start{i}");
        let end_i = format_ident!("end{i}");
        let start_next = format_ident!("start{}", i + 1);

        let line = quote!(let #end_i = #start_i + <#field_type>::BYTE_SIZE;);
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
            const ALIGNMENT: usize = #alignment;
            const BYTE_SIZE: usize = #byte_size;
            const NUM_ARGS: usize = #num_args;

            type StructLayout = [usize; #field_count_token * 2 + 1];

            fn layout() -> Self::StructLayout {
                let align = Self::ALIGNMENT;
                let start0 = 0;
                #layout_impl
                [start0, #layout_return]
            }
        }
      }
    )
}

pub fn size_description_enum(name: Ident, data: DataEnum) -> TokenStream {
    if data.variants.len() > 256 {
        return quote!(compile_error!(
            "Enums with more than 256 values are not yet supported."
        ));
    }

    quote!(
        impl wasm_bridge::direct_bytes::SizeDescription for #name {
            const NUM_ARGS: usize = 1;
            const ALIGNMENT: usize = 1;
            const BYTE_SIZE: usize = 1;

            type StructLayout = [usize; 3];

            fn layout() -> Self::StructLayout {
                [0, 1, 1]
            }
        }
    )
}

pub fn size_description_variant(name: Ident, data: DataEnum) -> TokenStream {
    let variants = data.variants;
    if variants.len() > 256 {
        return quote!(compile_error!(
            "Variants with more than 256 values are not yet supported."
        ));
    }

    let mut alignment = quote!(1usize);
    for variant in variants.iter() {
        if let Some(field) = variant.fields.iter().next() {
            let field_type = &field.ty;
            alignment = quote!(wasm_bridge::usize_max(#alignment, <#field_type>::ALIGNMENT));
        }
    }

    let mut byte_size = quote!(0usize);
    for variant in variants.iter() {
        if let Some(field) = variant.fields.iter().next() {
            let field_type = &field.ty;
            byte_size = quote!(wasm_bridge::usize_max(#byte_size, <#field_type>::BYTE_SIZE));
        }
    }

    let mut num_args = quote!(0usize);
    for variant in variants.iter() {
        if let Some(field) = variant.fields.iter().next() {
            let field_type = &field.ty;
            num_args = quote!(wasm_bridge::usize_max(#num_args, <#field_type>::NUM_ARGS));
        }
    }

    quote!(
        impl wasm_bridge::direct_bytes::SizeDescription for #name {
            const ALIGNMENT: usize = #alignment;
            const BYTE_SIZE: usize = Self::ALIGNMENT + #byte_size;
            const NUM_ARGS: usize = 1 + #num_args;

            type StructLayout = [usize; 3];

            fn layout() -> Self::StructLayout {
                [0, 1, 1]
            }
        }
    )
}

// TODO: remove?
fn num_to_token(num: usize) -> TokenStream {
    TokenStream::from_str(&num.to_string()).unwrap()
}
