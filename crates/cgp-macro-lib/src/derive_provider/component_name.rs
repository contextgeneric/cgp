use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{parse2, Error, Ident, ItemImpl, Type};

pub fn derive_component_name_from_provider_impl(provider_impl: &ItemImpl) -> syn::Result<Type> {
    let provider_trait = provider_impl.trait_.as_ref().ok_or_else(|| {
        Error::new(
            provider_impl.span(),
            "expect provider trait name to be present",
        )
    })?;

    let provider_ident: Ident = parse2(provider_trait.1.to_token_stream())?;

    let component_ident = Ident::new(&format!("{provider_ident}Component"), provider_ident.span());

    parse2(component_ident.to_token_stream())
}
