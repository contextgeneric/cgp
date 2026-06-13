use cgp_macro_core::functions::trait_items_to_delegated_impl_items;
use cgp_macro_core::types::attributes::DeriveDelegateAttribute;
use proc_macro2::Span;
use quote::quote;
use syn::token::{Brace, For, Impl};
use syn::{Ident, ItemImpl, ItemTrait, Path, parse_quote, parse2};

pub fn derive_delegate_impl(
    provider_trait: &ItemTrait,
    spec: &DeriveDelegateAttribute,
) -> syn::Result<ItemImpl> {
    let provider_trait_ident = &provider_trait.ident;

    let components_ident = Ident::new("__Components__", Span::call_site());
    let delegate_ident = Ident::new("__Delegate__", Span::call_site());

    let wrapper_ident = &spec.wrapper;
    let use_delegate_params = &spec.params;

    let generics = {
        let mut generics = provider_trait.generics.clone();

        generics.params.push(parse2(quote!( #components_ident ))?);
        generics.params.push(parse2(quote!( #delegate_ident ))?);

        let where_clause = generics.make_where_clause();

        where_clause.predicates.push(parse2(quote! {
            #components_ident: DelegateComponent<
                ( #use_delegate_params ),
                Delegate = #delegate_ident,
            >
        })?);

        let type_generics = provider_trait.generics.split_for_impl().1;

        where_clause.predicates.push(parse2(quote! {
            #delegate_ident : #provider_trait_ident #type_generics
        })?);

        generics
    };

    let type_generics = provider_trait.generics.split_for_impl().1;

    let trait_path: Path = parse_quote!( #provider_trait_ident #type_generics );

    let impl_items = trait_items_to_delegated_impl_items(
        &provider_trait.items,
        &parse_quote!( #delegate_ident ),
        &parse_quote!( #provider_trait_ident #type_generics ),
    )?;

    let provider_type = parse2(quote!(#wrapper_ident < #components_ident >))?;

    let item = ItemImpl {
        attrs: provider_trait.attrs.clone(),
        defaultness: None,
        unsafety: provider_trait.unsafety,
        impl_token: Impl::default(),
        generics,
        trait_: Some((None, trait_path, For::default())),
        self_ty: Box::new(provider_type),
        brace_token: Brace::default(),
        items: impl_items,
    };

    Ok(item)
}
