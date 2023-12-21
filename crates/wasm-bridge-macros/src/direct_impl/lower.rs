use heck::{ToKebabCase, ToLowerCamelCase};
use proc_macro2::*;
use quote::{quote, TokenStreamExt};
use syn::{DataEnum, DataStruct};

pub fn lower_struct(name: Ident, data: DataStruct) -> TokenStream {
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

            fn into_return_abi(self) -> Result<Self::ReturnAbi, wasm_bridge::wasm_bindgen::JsValue> {
                Ok(self.to_js_value())
            }
        }
    }
}

pub fn lower_enum(name: Ident, data: DataEnum) -> TokenStream {
    todo!()
}

pub fn lower_variant(name: Ident, data: DataEnum) -> TokenStream {
    todo!()
}
