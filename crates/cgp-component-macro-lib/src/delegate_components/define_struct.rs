use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, GenericParam, Generics, Ident, ItemStruct};

pub fn define_struct(ident: &Ident, generics: &Generics) -> ItemStruct {
    if generics.params.is_empty() {
        parse_quote! {
            pub struct #ident;
        }
    } else {
        let mut generic_params = generics.params.clone();
        let mut phantom_params: Punctuated<Ident, Comma> = Default::default();

        for param in generic_params.iter_mut() {
            if let GenericParam::Type(type_param) = param {
                type_param.colon_token = None;
                type_param.bounds.clear();

                phantom_params.push(type_param.ident.clone());
            }
        }

        parse_quote! {
            pub struct #ident < #generic_params > (
                pub ::core::marker::PhantomData<( #phantom_params )>
            );
        }
    }
}
