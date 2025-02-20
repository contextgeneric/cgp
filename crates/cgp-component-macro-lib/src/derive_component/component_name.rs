use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Ident, ItemStruct, parse_quote};

pub fn derive_component_name_struct(
    component_name: &Ident,
    component_params: &Punctuated<Ident, Comma>,
) -> ItemStruct {
    if component_params.is_empty() {
        parse_quote!(pub struct #component_name ;)
    } else {
        parse_quote!(pub struct #component_name < #component_params > ( pub core::marker::PhantomData<( #component_params )> );)
    }
}
