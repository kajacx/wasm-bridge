use proc_macro2::*;
use syn::{DataEnum, DataStruct};

pub fn size_description_struct(_name: Ident, _data: DataStruct) -> TokenStream {
    TokenStream::new()
}

pub fn size_description_enum(_name: Ident, _data: DataEnum) -> TokenStream {
    TokenStream::new()
}

pub fn size_description_variant(_name: Ident, _data: DataEnum) -> TokenStream {
    TokenStream::new()
}
