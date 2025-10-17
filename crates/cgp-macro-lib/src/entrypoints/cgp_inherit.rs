use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse2};

use crate::derive_context::derive_delegate_preset;
use crate::parse::{SimpleType, TypeGenerics};

pub fn cgp_inherit(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let context_struct: ItemStruct = parse2(body)?;

    let preset: SimpleType = parse2(attr)?;

    let type_generics = TypeGenerics::try_from(&context_struct.generics)?;

    let (delegate_impl, is_provider_impl) =
        derive_delegate_preset(&context_struct.ident, &Some(type_generics), &preset)?;

    Ok(quote! {
        #context_struct

        #delegate_impl

        #is_provider_impl
    })
}
