use syn::visit_mut::VisitMut;
use syn::{
    Error, Ident, ImplItem, ImplItemConst, ImplItemFn, ImplItemType, Item, ItemImpl, ItemTrait,
    Path, TraitItem, Type, Visibility, WherePredicate,
};

use crate::parse_internal;
use crate::visitors::RemoveSelfPathVisitor;

pub struct ItemBlanketTrait {
    pub context_ident: Ident,
    pub item_trait: ItemTrait,
}

impl ItemBlanketTrait {
    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let item_trait = self.item_trait.clone();
        let item_impl = self.to_item_impl()?;

        Ok(vec![item_trait.into(), item_impl.into()])
    }

    pub fn to_item_impl(&self) -> syn::Result<ItemImpl> {
        let context_ident = &self.context_ident;
        let mut item_trait = self.item_trait.clone();

        let mut impl_items: Vec<ImplItem> = Vec::new();

        let mut assoc_idents: Vec<Ident> = Vec::new();
        let mut assoc_bounds: Vec<WherePredicate> = Vec::new();

        for trait_item in item_trait.items.iter() {
            if let TraitItem::Type(trait_item_type) = trait_item {
                let item_type_ident = &trait_item_type.ident;
                assoc_idents.push(item_type_ident.clone());
            }
        }

        RemoveSelfPathVisitor {
            assoc_idents: &assoc_idents,
        }
        .visit_item_trait_mut(&mut item_trait);

        for trait_item in item_trait.items.iter_mut() {
            match trait_item {
                TraitItem::Type(trait_item_type) => {
                    trait_item_type.default.take();

                    let item_type_ident = &trait_item_type.ident;

                    let type_impl = parse_internal! {
                        #item_type_ident
                    };

                    if !trait_item_type.bounds.is_empty() {
                        let current_assoc_bounds = trait_item_type.bounds.clone();

                        assoc_bounds.push(parse_internal! {
                            #item_type_ident : #current_assoc_bounds
                        });
                    }

                    let impl_item_type = ImplItemType {
                        attrs: trait_item_type.attrs.clone(),
                        vis: Visibility::Inherited,
                        defaultness: None,
                        type_token: trait_item_type.type_token,
                        ident: trait_item_type.ident.clone(),
                        generics: trait_item_type.generics.clone(),
                        eq_token: Default::default(),
                        ty: type_impl,
                        semi_token: Default::default(),
                    };

                    impl_items.push(ImplItem::Type(impl_item_type));
                }
                TraitItem::Fn(trait_item_fn) => {
                    let fn_block = trait_item_fn
                        .default
                        .as_ref()
                        .ok_or_else(|| {
                            Error::new_spanned(
                                &trait_item_fn,
                                "function item require implementation block",
                            )
                        })?
                        .clone();

                    trait_item_fn.default.take();

                    let impl_item_fn = ImplItemFn {
                        attrs: trait_item_fn.attrs.clone(),
                        vis: Visibility::Inherited,
                        defaultness: None,
                        sig: trait_item_fn.sig.clone(),
                        block: fn_block,
                    };

                    impl_items.push(ImplItem::Fn(impl_item_fn));
                }
                TraitItem::Const(trait_item_const) => {
                    let (eq_token, const_expr) = trait_item_const
                        .default
                        .as_ref()
                        .ok_or_else(|| {
                            Error::new_spanned(
                                &trait_item_const,
                                "const item require implementation expression",
                            )
                        })?
                        .clone();

                    trait_item_const.default.take();

                    let impl_item_const = ImplItemConst {
                        attrs: trait_item_const.attrs.clone(),
                        vis: Visibility::Inherited,
                        defaultness: None,
                        const_token: trait_item_const.const_token,
                        ident: trait_item_const.ident.clone(),
                        generics: trait_item_const.generics.clone(),
                        colon_token: trait_item_const.colon_token,
                        ty: trait_item_const.ty.clone(),
                        eq_token,
                        expr: const_expr,
                        semi_token: trait_item_const.semi_token,
                    };

                    impl_items.push(ImplItem::Const(impl_item_const));
                }
                _ => return Err(Error::new_spanned(&trait_item, "unsupported trait item")),
            }
        }

        let context_type: Type = parse_internal!(#context_ident);

        let mut impl_generics = item_trait.generics.clone();

        impl_generics.params.push(parse_internal!(#context_type));

        for assoc_ident in assoc_idents.iter() {
            impl_generics.params.push(parse_internal!(#assoc_ident));
        }

        let supertraits = item_trait.supertraits.clone();

        let where_clause = impl_generics.make_where_clause();
        where_clause.predicates.push(parse_internal! {
            #context_type: #supertraits
        });

        where_clause.predicates.extend(assoc_bounds);

        let trait_name = &item_trait.ident;
        let (_, type_generics, _) = item_trait.generics.split_for_impl();

        let trait_path: Path = parse_internal! {
            #trait_name #type_generics
        };

        let item_impl = ItemImpl {
            attrs: item_trait.attrs.clone(),
            defaultness: None,
            unsafety: item_trait.unsafety,
            impl_token: Default::default(),
            generics: impl_generics,
            trait_: Some((None, trait_path, Default::default())),
            self_ty: Box::new(context_type),
            brace_token: item_trait.brace_token,
            items: impl_items,
        };

        Ok(item_impl)
    }
}
