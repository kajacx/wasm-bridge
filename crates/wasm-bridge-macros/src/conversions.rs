use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DataStruct, Ident, Path};

pub fn derive_component_type_struct(name: &Ident, data: DataStruct) -> TokenStream {
    let stride = data.fields.iter().map(|field| {
        let ty = &field.ty;
        quote! {
            <#ty as wasm_bridge::component::ComponentType>::STRIDE
        }
    });

    quote! {
        impl wasm_bridge::component::ComponentType for #name {
            const STRIDE: usize = #(#stride)+*;
        }
    }
}

pub fn derive_lower_struct(name: &Ident, data: DataStruct) -> TokenStream {
    // Write each field sequentially into the encoder
    let lower_field = data.fields.iter().map(|field| {
        let ty = &field.ty;
        let ident = &field.ident;
        quote! {
            <#ty as wasm_bridge::component::Lower>::lower(&self.#ident, cx, encoder);
            encoder.advance(<#ty as wasm_bridge::component::ComponentType>::STRIDE);
        }
    });

    let lower_field_arg = data.fields.iter().map(|field| {
        let ty = &field.ty;
        let ident = &field.ident;

        quote! {
            <#ty as wasm_bridge::component::Lower>::lower_args(&self.#ident, cx, dst);
        }
    });

    quote! {
        // impl wasm_bridge::component::Lower for #name {
        //     fn lower(&self, cx: &wasm_bridge::component::LowerContext, encoder: &mut wasm_bridge::component::Encoder<'_>) {
        //         #(#lower_field)*
        //     }

        //     fn lower_args(&self, cx: &wasm_bridge::component::LowerContext, dst: &mut wasm_bridge::js_sys::Array) {
        //         #(#lower_field_arg)*
        //     }
        // }
    }
}

pub fn derive_lift_struct(name: &Ident, data: DataStruct) -> TokenStream {
    quote! {}
}
