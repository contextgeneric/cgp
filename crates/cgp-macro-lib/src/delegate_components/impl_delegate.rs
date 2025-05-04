use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse2, ImplItem, ImplItemType, ItemImpl, Path, Type};

use crate::delegate_components::define_struct;
use crate::delegate_components::merge_generics::merge_generics;
use crate::parse::{DelegateEntry, DelegateKey, DelegateValue, ImplGenerics};

pub fn impl_delegate_components<T>(
    target_type: &Type,
    target_generics: &ImplGenerics,
    delegate_entries: &Punctuated<DelegateEntry<T>, Comma>,
) -> syn::Result<TokenStream>
where
    T: ToTokens,
{
    let mut out = TokenStream::new();

    for entry in delegate_entries.iter() {
        let source = &entry.value;
        for component in entry.keys.iter() {
            let impls = impl_delegate_component(target_type, target_generics, component, source)?;

            out.extend(impls);
        }
    }

    Ok(out)
}

pub fn impl_delegate_component<T>(
    target_type: &Type,
    target_generics: &ImplGenerics,
    component: &DelegateKey<T>,
    value: &DelegateValue,
) -> syn::Result<TokenStream>
where
    T: ToTokens,
{
    let component_type = &component.ty;

    let value_type = value.as_type();

    let delegate_trait_path: Path = parse2(quote!(DelegateComponent < #component_type >))?;

    let delegate_type: ImplItemType = parse2(quote!(type Delegate = #value_type;))?;

    let delegate_generics = merge_generics(&target_generics.generics, &component.generics.generics);

    let is_provider_generics = {
        let mut generics = delegate_generics.clone();

        generics.params.push(parse2(quote!(__Context__))?);
        generics.params.push(parse2(quote!(__Params__))?);

        let where_clause = generics.make_where_clause();
        where_clause.predicates.push(parse2(
            quote!( #value_type : IsProviderFor< #component_type, __Context__, __Params__ > ),
        )?);

        generics
    };

    let delegate_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: delegate_generics,
        trait_: Some((None, delegate_trait_path, Default::default())),
        self_ty: Box::new(target_type.clone()),
        brace_token: Default::default(),
        items: vec![ImplItem::Type(delegate_type)],
    };

    let is_provider_trait_path: Path =
        parse2(quote!( IsProviderFor< #component_type, __Context__, __Params__ > ))?;

    let is_provider_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: is_provider_generics,
        trait_: Some((None, is_provider_trait_path, Default::default())),
        self_ty: Box::new(target_type.clone()),
        brace_token: Default::default(),
        items: Default::default(),
    };

    let mut out = quote! {
        #delegate_impl
        #is_provider_impl
    };

    if let DelegateValue::New(value) = value {
        let struct_ident = &value.struct_ident;

        let item_struct = define_struct(struct_ident, &value.struct_generics)?;

        let (impl_generics, type_generics, _) = value.struct_generics.split_for_impl();

        let target_type: Type = parse2(quote! { #struct_ident #type_generics })?;

        let impl_generics = parse2(quote! { #impl_generics })?;

        let inner_impls = impl_delegate_components(&target_type, &impl_generics, &value.entries)?;

        out.extend(item_struct.to_token_stream());
        out.extend(inner_impls);
    }

    Ok(out)
}
