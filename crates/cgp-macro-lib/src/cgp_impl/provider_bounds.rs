use quote::{ToTokens, quote};
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{Type, TypeParamBound, WherePredicate, parse_quote, parse2};

use crate::cgp_impl::use_provider::UseProviderSpec;

pub fn derive_provider_bounds(
    context_type: &Type,
    spec: &UseProviderSpec,
) -> syn::Result<WherePredicate> {
    let context_type = if spec.context_type == parse_quote! { Self } {
        context_type
    } else {
        &spec.context_type
    };

    let provider_type = &spec.provider_type;
    let mut bounds = Punctuated::<TypeParamBound, Plus>::new();

    for bound in &spec.provider_trait_bounds {
        let trait_ident = &bound.name;
        let mut m_generics = bound.generics.clone();

        let generics = m_generics.get_or_insert_with(|| parse_quote!(<>));
        generics
            .args
            .insert(0, parse2(context_type.to_token_stream())?);

        let trait_bound = parse2(quote! {
            #trait_ident #generics
        })?;
        bounds.push(trait_bound);
    }

    let predicate = parse2(quote! {
        #provider_type: #bounds
    })?;

    Ok(predicate)
}
