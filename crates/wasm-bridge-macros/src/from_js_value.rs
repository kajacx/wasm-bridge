use heck::{ToKebabCase, ToLowerCamelCase};
use proc_macro2::*;
use quote::{quote, TokenStreamExt};
use syn::{DataEnum, DataStruct};

pub fn from_js_value_struct(name: Ident, data: DataStruct) -> TokenStream {
    let mut impl_block = TokenStream::new();
    let mut fields_constructor = TokenStream::new();

    for field in data.fields {
        let field_type = field.ty;
        let field_name = field.ident;

        let field_name_str = quote!(#field_name).to_string();
        let field_name_converted = field_name_str.to_lower_camel_case();

        let tokens = quote!(
            let js_field = wasm_bridge::js_sys::Reflect::get(value, &#field_name_converted.into())?;
            let #field_name = #field_type::from_js_value(&js_field)?;
        );
        impl_block.append_all(tokens);

        fields_constructor.append_all(quote!(#field_name, ));
    }

    quote! {
        impl wasm_bridge::FromJsValue for #name {
            type WasmAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn from_js_value(value: &wasm_bridge::wasm_bindgen::JsValue) -> wasm_bridge::Result<Self> {
                #impl_block

                Ok(Self { #fields_constructor })
            }

            fn from_wasm_abi(abi: Self::WasmAbi) -> wasm_bridge::Result<Self> {
                Self::from_js_value(&abi)
            }
        }
    }
}

pub fn from_js_value_enum(name: Ident, data: DataEnum) -> TokenStream {
    let mut impl_block = TokenStream::new();

    for variant in data.variants {
        let variant_name = variant.ident;
        let variant_name_str = quote!(#variant_name).to_string();
        let variant_name_converted = variant_name_str.to_kebab_case();

        let field = variant.fields.iter().next();

        let return_value = match field {
            Some(field) => {
                let field_type = &field.ty;
                quote!( Self::#variant_name(<#field_type>::from_js_value(&val)?) )
            }
            None => quote!( Self::#variant_name ),
        };

        let tokens = quote!(
            if val == #variant_name_converted {
                return Ok(#return_value);
            };
        );
        impl_block.append_all(tokens);
    }

    quote! {
        impl wasm_bridge::FromJsValue for #name {
            type WasmAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn from_js_value(value: &wasm_bridge::wasm_bindgen::JsValue) -> wasm_bridge::Result<Self> {
                let tag = wasm_bridge::js_sys::Reflect::get(value, &"tag".into())?;
                let tag = tag.as_string().ok_or(value)?;

                let val = wasm_bridge::js_sys::Reflect::get(value, &"val".into())?;

                #impl_block

                // TODO: better user error
                Err(value.into())
            }

            fn from_wasm_abi(abi: Self::WasmAbi) -> wasm_bridge::Result<Self> {
                Self::from_js_value(&abi)
            }
        }
    }
}
