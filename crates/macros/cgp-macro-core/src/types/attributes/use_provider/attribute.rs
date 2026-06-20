use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Plus};
use syn::{Type, TypeParamBound, WherePredicate};

use crate::parse_internal;
use crate::types::ident::PathWithTypeArgs;

pub struct UseProviderAttribute {
    pub context_type: Type,
    pub provider_type: Type,
    pub colon: Colon,
    pub provider_trait_bounds: Punctuated<PathWithTypeArgs, Plus>,
}

impl UseProviderAttribute {
    pub fn to_type_param_bounds(
        &self,
        context_type: &Type,
    ) -> syn::Result<Punctuated<TypeParamBound, Plus>> {
        let mut bounds = Punctuated::<TypeParamBound, Plus>::new();

        for bound in &self.provider_trait_bounds {
            let mut bound = bound.clone();
            bound
                .type_args
                .args
                .insert(0, parse_internal!(#context_type));

            bounds.push(parse_internal!(#bound));
        }

        Ok(bounds)
    }

    pub fn to_provider_bounds(&self, context_type: &Type) -> syn::Result<WherePredicate> {
        let provider_type = &self.provider_type;
        let bounds = self.to_type_param_bounds(context_type)?;

        let predicate = parse_internal! {
            #provider_type: #bounds
        };

        Ok(predicate)
    }
}

impl Parse for UseProviderAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let context_type = parse_internal!(Self);
        let provider_type = input.parse()?;

        let colon: Colon = input.parse()?;
        let provider_trait_bounds = Punctuated::parse_terminated(input)?;

        Ok(Self {
            context_type,
            provider_type,
            colon,
            provider_trait_bounds,
        })
    }
}
