use cgp_macro_core::types::attributes::UseProviderAttribute;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{Type, TypeParamBound, WherePredicate, parse_quote, parse2};

pub fn derive_provider_bounds(
    context_type: &Type,
    spec: &UseProviderAttribute,
) -> syn::Result<WherePredicate> {
    let context_type = if spec.context_type == parse_quote! { Self } {
        context_type
    } else {
        &spec.context_type
    };

    let provider_type = &spec.provider_type;
    let mut bounds = Punctuated::<TypeParamBound, Plus>::new();

    for bound in &spec.provider_trait_bounds {
        let mut bound = bound.clone();
        bound
            .type_args
            .make_args()
            .insert(0, parse_quote!(#context_type));

        bounds.push(parse_quote!(#bound));
    }

    let predicate = parse2(quote! {
        #provider_type: #bounds
    })?;

    Ok(predicate)
}
