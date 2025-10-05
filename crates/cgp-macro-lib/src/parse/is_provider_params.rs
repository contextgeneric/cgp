use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{GenericParam, Generics, Type, parse_quote};

use crate::parse::TypeGenerics;

pub fn parse_is_provider_params(generics: &Generics) -> syn::Result<Punctuated<Type, Comma>> {
    let params = TypeGenerics::try_from(generics)?.generics.params;

    let params = params.into_iter().map(|param| -> Type {
        match param {
            GenericParam::Type(type_param) => {
                let ident = type_param.ident;
                parse_quote! { #ident }
            }
            GenericParam::Lifetime(life_param) => {
                let life = &life_param.lifetime;
                parse_quote! { Life<#life> }
            }
            GenericParam::Const(_) => {
                unimplemented!("const generic parameters are not yet supported in CGP traits")
            }
        }
    });

    Ok(Punctuated::from_iter(params))
}
