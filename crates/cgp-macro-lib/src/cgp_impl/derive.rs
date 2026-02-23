use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::visit_mut::visit_item_impl_mut;
use syn::{ItemImpl, TypeParamBound, parse2};

use crate::cgp_fn::{apply_use_type_attributes_to_item_impl, build_implicit_args_bounds};
use crate::cgp_impl::attributes::parse_impl_attributes;
use crate::cgp_impl::provider_bounds::derive_provider_bounds;
use crate::cgp_impl::provider_call::TransformProviderCallVisitor;
use crate::cgp_impl::{ImplProviderSpec, derive_provider_impl, implicit_args};
use crate::derive_provider::{
    derive_component_name_from_provider_impl, derive_is_provider_for, derive_provider_struct,
};

pub fn derive_cgp_impl(
    spec: ImplProviderSpec,
    mut item_impl: ItemImpl,
) -> syn::Result<TokenStream> {
    let attributes = parse_impl_attributes(&mut item_impl.attrs)?;

    let implicit_args = implicit_args::extract_implicit_args_from_impl_items(&mut item_impl.items)?;

    if !implicit_args.is_empty() {
        let where_clause = item_impl.generics.make_where_clause();
        let bounds = build_implicit_args_bounds(&implicit_args)?;

        where_clause.predicates.push(parse2(quote! {
            Self: #bounds
        })?);
    }

    if !attributes.use_type.is_empty() {
        item_impl = apply_use_type_attributes_to_item_impl(&item_impl, &attributes.use_type)?;
    }

    if !attributes.uses.is_empty() {
        let mut bounds: Punctuated<TypeParamBound, Plus> = Punctuated::default();

        for import in attributes.uses.iter() {
            bounds.push(parse2(quote! { #import })?);
        }

        item_impl
            .generics
            .make_where_clause()
            .predicates
            .push(parse2(quote! {
                Self: #bounds
            })?);
    }

    let mut visitor = TransformProviderCallVisitor::default();
    visit_item_impl_mut(&mut visitor, &mut item_impl);
    if let Some(err) = visitor.error {
        return Err(err);
    }

    let (context_type, mut provider_impl) = derive_provider_impl(&spec.provider_type, item_impl)?;

    let component_type = match &spec.component_type {
        Some(component_type) => component_type.clone(),
        None => derive_component_name_from_provider_impl(&provider_impl)?,
    };

    let is_provider_for_impl: ItemImpl = derive_is_provider_for(&component_type, &provider_impl)?;

    if !attributes.use_provider.is_empty() {
        let where_clause = provider_impl.generics.make_where_clause();

        for spec in attributes.use_provider.iter() {
            let provider_bounds = derive_provider_bounds(&context_type, spec)?;
            where_clause.predicates.push(provider_bounds);
        }
    }

    let provider_struct = if spec.new_struct {
        Some(derive_provider_struct(&provider_impl)?)
    } else {
        None
    };

    Ok(quote! {
        #provider_struct

        #provider_impl

        #is_provider_for_impl
    })
}
