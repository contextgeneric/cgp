use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{GenericParam, Generics, Type};

use crate::exports::Life;
use crate::parse_internal;
use crate::types::generics::TypeGenerics;

/// Convert a trait's generics into the `Params` tuple types of an `IsProviderFor`
/// bound: type params pass through, lifetimes are lifted into `Life<'a>`.
///
/// Panics on a const generic parameter — see the const-generic limitation in
/// docs/implementation/entrypoints/cgp_component.md.
pub fn parse_is_provider_params(generics: &Generics) -> syn::Result<Punctuated<Type, Comma>> {
    let params = TypeGenerics::try_from(generics)?.generics.params;

    let mut res = Punctuated::new();

    for param in params {
        let out = match param {
            GenericParam::Type(type_param) => {
                let ident = type_param.ident;
                parse_internal! { #ident }
            }
            GenericParam::Lifetime(life_param) => {
                let life = &life_param.lifetime;
                parse_internal! { #Life<#life> }
            }
            GenericParam::Const(_) => {
                unimplemented!("const generic parameters are not yet supported in CGP traits")
            }
        };
        res.push(out)
    }

    Ok(res)
}
