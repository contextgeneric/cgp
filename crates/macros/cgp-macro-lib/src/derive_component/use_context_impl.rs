use cgp_macro_core::functions::trait_items_to_delegated_impl_items;
use cgp_macro_core::types::generics::TypeGenerics;
use quote::quote;
use syn::token::{Brace, For, Impl};
use syn::{Ident, ItemImpl, ItemTrait, Path, Type, parse_quote, parse2};

pub fn derive_use_context_impl(
    context_type_ident: &Ident,
    consumer_trait: &ItemTrait,
    provider_trait: &ItemTrait,
) -> syn::Result<ItemImpl> {
    let consumer_trait_ident = &consumer_trait.ident;
    let provider_trait_ident = &provider_trait.ident;

    let provider_generics = TypeGenerics::try_from(&provider_trait.generics)?.generics;

    let consumer_generics = TypeGenerics::try_from(&consumer_trait.generics)?.generics;

    let mut impl_generics = provider_trait.generics.clone();

    let where_clause = impl_generics.make_where_clause();

    where_clause.predicates.push(parse2(quote! {
        #context_type_ident : #consumer_trait_ident #consumer_generics
    })?);

    let consumer_trait_ident = &consumer_trait.ident;
    let consumer_trait_generics = consumer_trait.generics.split_for_impl().1;
    let consumer_trait_path: Type = parse_quote!(#consumer_trait_ident #consumer_trait_generics);

    let impl_items = trait_items_to_delegated_impl_items(
        &provider_trait.items,
        &parse_quote!(#context_type_ident),
        &consumer_trait_path,
    )?;

    let provider_trait_path: Path = parse2(quote!( #provider_trait_ident #provider_generics ))?;

    let item = ItemImpl {
        attrs: provider_trait.attrs.clone(),
        defaultness: None,
        unsafety: provider_trait.unsafety,
        impl_token: Impl::default(),
        generics: impl_generics,
        trait_: Some((None, provider_trait_path, For::default())),
        self_ty: Box::new(parse2(quote!(UseContext))?),
        brace_token: Brace::default(),
        items: impl_items,
    };

    Ok(item)
}
