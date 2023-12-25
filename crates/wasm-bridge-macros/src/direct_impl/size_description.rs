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
                let align = 1;
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

pub fn size_description_enum(name: Ident, data: DataEnum) -> TokenStream {
    if data.variants.len() > 256 {
        return quote!(compile_error!(
            "Enums with more than 256 values are not yet supported."
        ));
    }

    quote!(
        impl wasm_bridge::direct_bytes::SizeDescription for #name {
            type StructLayout = [usize; 3];

            fn alignment() -> usize {
                1
            }

            fn flat_byte_size() -> usize {
                1
            }

            fn num_args() -> usize {
                1
            }

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

    let mut alignment_impl = TokenStream::new();
    for variant in variants.iter() {
        let field = variant.fields.iter().next();
        if let Some(field) = field {
            let field_type = &field.ty;
            let line = quote!(let align = usize::max(align, <#field_type>::alignment()););
            alignment_impl.extend(line);
        }
    }

    let flat_byte_size: TokenStream =
        variants
            .iter()
            .map(|variant| {
                variant.fields.iter().next().map(|field| {
                let field_type = &field.ty;
                quote!(let max_size = usize::max(max_size, <#field_type>::flat_byte_size());)
             }).unwrap_or_else(TokenStream::new)
            })
            .collect();

    let num_args: TokenStream = variants
        .iter()
        .map(|variant| {
            variant
                .fields
                .iter()
                .next()
                .map(|field| {
                    let field_type = &field.ty;
                    quote!(let num_args = usize::max(num_args, <#field_type>::num_args());)
                })
                .unwrap_or_else(TokenStream::new)
        })
        .collect();

    quote!(
        impl wasm_bridge::direct_bytes::SizeDescription for #name {
            type StructLayout = [usize; 3];

            fn alignment() -> usize {
                let align = 1;
                #alignment_impl
                align
            }

            fn flat_byte_size() -> usize {
                let max_size = 0;
                #flat_byte_size
                Self::alignment() + max_size
            }

            fn num_args() -> usize {
                let num_args = 0;
                #num_args
                1 + num_args
            }

            fn layout() -> Self::StructLayout {
                [0, 1, 1]
            }
        }
    )
}

fn num_to_token(num: usize) -> TokenStream {
    TokenStream::from_str(&num.to_string()).unwrap()
}
