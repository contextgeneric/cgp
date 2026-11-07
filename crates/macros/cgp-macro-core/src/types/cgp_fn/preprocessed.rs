use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{
    Generics, Ident, Item, ItemFn, ItemImpl, ItemTrait, TraitItemFn, Type, TypeParamBound,
    Visibility, parse_quote,
};

use crate::functions::parse_internal;
use crate::traits::AddTypeParamBounds;
use crate::types::attributes::FunctionAttributes;
use crate::types::implicits::ImplicitArgFields;

/// The checked `#[cgp_fn]` input: the function, its implicits, and its attributes.
pub struct PreprocessedItemCgpFn {
    /// The trait's name.
    pub ident: Ident,
    /// The annotated function.
    pub item_fn: ItemFn,
    /// The `#[implicit]` arguments.
    pub implicit_args: ImplicitArgFields,
    /// The function's modifier attributes.
    pub attributes: FunctionAttributes,
    /// The function's visibility.
    pub visibility: Visibility,
    /// The function's generics.
    pub generics: Generics,
}

impl PreprocessedItemCgpFn {
    /// Generates the trait, the blanket impl, and the companion items.
    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let item_trait = self.to_item_trait()?;
        let item_impl = self.to_item_impl()?;

        Ok(vec![item_trait.into(), item_impl.into()])
    }

    /// Generates the trait the function becomes.
    pub fn to_item_trait(&self) -> syn::Result<ItemTrait> {
        let Self {
            ident,
            item_fn,
            attributes,
            generics,
            visibility,
            ..
        } = self;

        // The function's doc comment describes the operation, so it stays on the
        // method, where the function's body lands. The trait gets a line of its
        // own, and the method gets one when the function carries no doc, so a
        // crate that enables `missing_docs` reports neither.
        let (doc_attrs, other_attrs): (Vec<_>, Vec<_>) = attributes
            .raw_attributes
            .iter()
            .cloned()
            .partition(|attr| attr.path().is_ident("doc"));

        let mut method_attrs = item_fn.attrs.clone();
        method_attrs.extend(doc_attrs);
        if !method_attrs.iter().any(|attr| attr.path().is_ident("doc")) {
            let fn_ident = &item_fn.sig.ident;
            let method_doc = format!("The `{fn_ident}` operation.");
            method_attrs.push(parse_quote!(#[doc = #method_doc]));
        }

        let trait_item_fn = TraitItemFn {
            attrs: method_attrs,
            sig: item_fn.sig.clone(),
            default: None,
            semi_token: None,
        };

        let mut item_trait: ItemTrait = parse_internal! {
            pub trait #ident {
                #trait_item_fn
            }
        };

        item_trait.generics = generics.clone();
        item_trait.generics.where_clause = None;

        item_trait.supertraits.extend(attributes.extend.clone());

        if !attributes.extend_where.is_empty() {
            item_trait
                .generics
                .make_where_clause()
                .predicates
                .extend(attributes.extend_where.clone());
        }

        attributes.use_type.transform_item_trait(&mut item_trait)?;

        item_trait.attrs.extend(other_attrs);
        let fn_ident = &item_fn.sig.ident;
        let trait_doc = format!("The trait for the `{fn_ident}` operation.");
        item_trait.attrs.push(parse_quote!(#[doc = #trait_doc]));
        item_trait.vis = visibility.clone();

        Ok(item_trait)
    }

    /// Generates the blanket impl that calls the function.
    pub fn to_item_impl(&self) -> syn::Result<ItemImpl> {
        let Self {
            ident,
            item_fn,
            implicit_args,
            attributes,
            generics,
            ..
        } = self;

        let type_generics = generics.split_for_impl().1;

        let self_type: Type = parse_internal!(Self);

        let mut item_impl: ItemImpl = parse_internal! {
            impl #ident #type_generics for __Context__ {
                #item_fn
            }
        };

        item_impl.generics = generics.clone();
        // Insert the context as the leading impl generic. Position 0 is safe with
        // a lifetime present because `syn::Generics::to_tokens` emits lifetimes
        // first. See cgp-knowledge-base/cgp/implementation/README.md, "Generic-parameter insertion
        // and lifetime ordering".
        item_impl
            .generics
            .params
            .insert(0, parse_internal!(__Context__));

        item_impl
            .generics
            .params
            .extend(attributes.impl_generics.clone());

        {
            let mut bounds: Punctuated<TypeParamBound, Plus> = Punctuated::default();
            bounds.extend(attributes.extend.clone());
            bounds.extend(attributes.uses.iter().cloned());

            if !bounds.is_empty() {
                item_impl
                    .generics
                    .make_where_clause()
                    .predicates
                    .push(parse_internal! {
                        Self: #bounds
                    });
            }
        }

        if !attributes.extend_where.is_empty() {
            item_impl
                .generics
                .make_where_clause()
                .predicates
                .extend(attributes.extend_where.clone());
        }

        implicit_args.add_type_param_bounds(&self_type, &mut item_impl.generics)?;

        attributes.use_type.transform_item_impl(&mut item_impl)?;
        attributes
            .use_provider
            .add_type_param_bounds(&self_type, &mut item_impl.generics)?;

        item_impl.attrs.extend(attributes.raw_attributes.clone());

        Ok(item_impl)
    }
}
