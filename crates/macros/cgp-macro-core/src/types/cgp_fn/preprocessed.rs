use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{
    Generics, Ident, Item, ItemFn, ItemImpl, ItemTrait, TraitItemFn, Type, TypeParamBound,
    Visibility,
};

use crate::functions::parse_internal;
use crate::traits::AddTypeParamBounds;
use crate::types::attributes::FunctionAttributes;
use crate::types::implicits::ImplicitArgFields;

pub struct PreprocessedItemCgpFn {
    pub ident: Ident,
    pub item_fn: ItemFn,
    pub implicit_args: ImplicitArgFields,
    pub attributes: FunctionAttributes,
    pub visibility: Visibility,
    pub generics: Generics,
}

impl PreprocessedItemCgpFn {
    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let item_trait = self.to_item_trait()?;
        let item_impl = self.to_item_impl()?;

        Ok(vec![item_trait.into(), item_impl.into()])
    }

    pub fn to_item_trait(&self) -> syn::Result<ItemTrait> {
        let Self {
            ident,
            item_fn,
            attributes,
            generics,
            visibility,
            ..
        } = self;

        let trait_item_fn = TraitItemFn {
            attrs: item_fn.attrs.clone(),
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

        item_trait.attrs.extend(attributes.raw_attributes.clone());
        item_trait.vis = visibility.clone();

        Ok(item_trait)
    }

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

            for import in attributes.uses.iter() {
                bounds.push(parse_internal! { #import });
            }

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
