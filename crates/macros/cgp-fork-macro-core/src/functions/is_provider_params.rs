use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Error, GenericParam, Generics, Type};

use crate::exports::Life;
use crate::parse_internal;

/// Convert a trait's generics into the `Params` tuple types of an `IsProviderFor`
/// bound: type parameters pass through by name and lifetimes are lifted into
/// `Life<'a>`. Bounds and defaults are dropped, since the tuple only names the
/// parameters positionally.
///
/// Const generic parameters are rejected with a spanned error: the tuple holds
/// *types*, and CGP's type-based wiring cannot key on a const value, so a const
/// parameter has no representation here.
pub fn parse_is_provider_params(generics: &Generics) -> syn::Result<Punctuated<Type, Comma>> {
    let mut res = Punctuated::new();

    for param in &generics.params {
        let out = match param {
            GenericParam::Type(type_param) => {
                let ident = &type_param.ident;
                parse_internal! { #ident }
            }
            GenericParam::Lifetime(life_param) => {
                let life = &life_param.lifetime;
                parse_internal! { #Life<#life> }
            }
            GenericParam::Const(const_param) => {
                return Err(Error::new_spanned(
                    const_param,
                    "const generic parameters are not supported on CGP component traits",
                ));
            }
        };
        res.push(out)
    }

    Ok(res)
}
