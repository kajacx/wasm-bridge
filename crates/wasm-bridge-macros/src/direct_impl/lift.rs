use heck::{ToKebabCase, ToLowerCamelCase};
use proc_macro2::*;
use quote::{quote, TokenStreamExt};
use syn::{DataEnum, DataStruct};

pub fn lift_struct(name: Ident, data: DataStruct) -> TokenStream {
    todo!()
}

pub fn lift_enum(name: Ident, data: DataEnum) -> TokenStream {
    todo!()
}

pub fn lift_variant(name: Ident, data: DataEnum) -> TokenStream {
    todo!()
}
