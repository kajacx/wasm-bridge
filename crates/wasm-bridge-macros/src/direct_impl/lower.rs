use heck::{ToKebabCase, ToLowerCamelCase};
use proc_macro2::*;
use quote::{quote, TokenStreamExt};
use syn::{DataEnum, DataStruct};

pub fn lower_struct(name: Ident, data: DataStruct) -> TokenStream {
    // todo!()
    TokenStream::new()
}

pub fn lower_enum(name: Ident, data: DataEnum) -> TokenStream {
    todo!()
}

pub fn lower_variant(name: Ident, data: DataEnum) -> TokenStream {
    todo!()
}
