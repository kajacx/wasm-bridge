use heck::{ToKebabCase, ToLowerCamelCase};
use proc_macro2::*;
use quote::{quote, TokenStreamExt};
use syn::{DataEnum, DataStruct};

pub fn to_js_value_struct(name: Ident, data: DataStruct) -> TokenStream {
    let mut impl_block = TokenStream::new();

    for field in data.fields {
        let field_name = field.ident;

        let field_name_str = quote!(#field_name).to_string();
        let field_name_converted = field_name_str.to_lower_camel_case();

        let tokens = quote!(
            wasm_bridge::js_sys::Reflect::set(
                &value,
                &#field_name_converted.into(),
                &self.#field_name.to_js_value(),
            ).expect("value is object");
        );

        impl_block.append_all(tokens);
    }

    quote! {
        impl wasm_bridge::ToJsValue for #name {
            type ReturnAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn to_js_value(&self) -> wasm_bridge::wasm_bindgen::JsValue {
                let value  = wasm_bridge::js_sys::Object::new();
                let value: wasm_bridge::wasm_bindgen::JsValue = value.into();

                #impl_block

                value
            }

            fn into_return_abi(self) -> Self::ReturnAbi {
                self.to_js_value()
            }
        }
    }
}

pub fn to_js_value_enum(name: Ident, data: DataEnum) -> TokenStream {
    let mut impl_block = TokenStream::new();

    for variant in data.variants {
        let variant_name = variant.ident;
        let variant_name_str = quote!(#variant_name).to_string();
        let variant_name_converted = variant_name_str.to_kebab_case();

        let return_value = quote!(
            Self::#variant_name => {
                #variant_name_converted.into()
            },
        );

        impl_block.append_all(return_value);
    }

    quote! {
        impl wasm_bridge::ToJsValue for #name {
            type ReturnAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn to_js_value(&self) -> wasm_bridge::wasm_bindgen::JsValue {
                match self {
                    #impl_block
                }
            }

            fn into_return_abi(self) -> Self::ReturnAbi {
                self.to_js_value()
            }
        }
    }
}

pub fn to_js_value_variant(name: Ident, data: DataEnum) -> TokenStream {
    let mut impl_block = TokenStream::new();

    for variant in data.variants {
        let variant_name = variant.ident;
        let variant_name_str = quote!(#variant_name).to_string();
        let variant_name_converted = variant_name_str.to_kebab_case();

        let create_result = quote!(
            let result = wasm_bridge::js_sys::Object::new();
            let result: wasm_bridge::wasm_bindgen::JsValue = result.into();
            wasm_bridge::js_sys::Reflect::set(&result, &"tag".into(), &#variant_name_converted.into()).expect("result is object");
        );

        let field = variant.fields.iter().next();
        let return_value = match field {
            Some(_) => quote!(
                Self::#variant_name(value) => {
                    #create_result
                    wasm_bridge::js_sys::Reflect::set(&result, &"val".into(), &value.to_js_value()).expect("result is object");
                    result
                }
            ),
            None => quote!(
                Self::#variant_name => {
                    #create_result
                    result
                }
            ),
        };

        impl_block.append_all(return_value);
    }

    quote! {
        impl wasm_bridge::ToJsValue for #name {
            type ReturnAbi = wasm_bridge::wasm_bindgen::JsValue;

            fn to_js_value(&self) -> wasm_bridge::wasm_bindgen::JsValue {
                match self {
                    #impl_block
                }
            }

            fn into_return_abi(self) -> Self::ReturnAbi {
                self.to_js_value()
            }
        }
    }
}
