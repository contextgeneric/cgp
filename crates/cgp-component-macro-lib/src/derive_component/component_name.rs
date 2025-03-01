use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse2, Ident, ItemStruct};

pub fn derive_component_name_struct(
    component_name: &Ident,
    component_params: &Punctuated<Ident, Comma>,
) -> syn::Result<ItemStruct> {
    if component_params.is_empty() {
        parse2(quote!(pub struct #component_name ;))
    } else {
        parse2(
            quote!(pub struct #component_name < #component_params > ( pub core::marker::PhantomData<( #component_params )> );),
        )
    }
}
