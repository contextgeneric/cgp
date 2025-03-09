use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::token::{Eq, For, Impl, Semi};
use syn::{
    parse2, Error, GenericArgument, Ident, ImplItem, ImplItemConst, ImplItemFn, ImplItemType,
    ItemImpl, ItemTrait, Path, PathArguments, TraitItem, Type, TypeParamBound, Visibility,
    WherePredicate,
};

pub fn derive_trait_alias(
    context_ident: &Ident,
    item_trait: &mut ItemTrait,
) -> syn::Result<ItemImpl> {
    let mut impl_items: Vec<ImplItem> = Vec::new();

    let mut assoc_bounds: Vec<WherePredicate> = Vec::new();

    for trait_item in item_trait.items.iter_mut() {
        match trait_item {
            TraitItem::Type(trait_item_type) => {
                let type_impl = trait_item_type
                    .default
                    .as_ref()
                    .ok_or_else(|| {
                        Error::new_spanned(
                            &trait_item_type,
                            "type item require assignment to concrete type",
                        )
                    })?
                    .1
                    .clone();

                trait_item_type.default.take();

                for bound in trait_item_type.bounds.iter() {
                    assoc_bounds.push(parse2(quote! {
                        #type_impl : #bound
                    })?);
                }

                let impl_item_type = ImplItemType {
                    attrs: trait_item_type.attrs.clone(),
                    vis: Visibility::Inherited,
                    defaultness: None,
                    type_token: trait_item_type.type_token,
                    ident: trait_item_type.ident.clone(),
                    generics: trait_item_type.generics.clone(),
                    eq_token: Eq(Span::call_site()),
                    ty: type_impl,
                    semi_token: Semi(Span::call_site()),
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

    let context_type: Type = parse2(quote! { #context_ident })?;

    let mut impl_generics = item_trait.generics.clone();
    impl_generics
        .params
        .push(parse2(context_type.to_token_stream())?);

    let mut supertraits = item_trait.supertraits.clone();

    for bound in supertraits.iter_mut() {
        if let TypeParamBound::Trait(trait_bound) = bound {
            filter_assoc_self_constraint(&mut trait_bound.path);
        }
    }

    let where_clause = impl_generics.make_where_clause();
    where_clause.predicates.push(parse2(quote! {
        #context_type: #supertraits
    })?);

    where_clause.predicates.extend(assoc_bounds);

    let trait_name = &item_trait.ident;
    let (_, type_generics, _) = item_trait.generics.split_for_impl();

    let trait_path: Path = parse2(quote! { #trait_name #type_generics })?;

    let item_impl = ItemImpl {
        attrs: item_trait.attrs.clone(),
        defaultness: None,
        unsafety: item_trait.unsafety,
        impl_token: Impl(Span::call_site()),
        generics: impl_generics,
        trait_: Some((None, trait_path, For(Span::call_site()))),
        self_ty: Box::new(context_type),
        brace_token: item_trait.brace_token,
        items: impl_items,
    };

    Ok(item_impl)
}

pub fn filter_assoc_self_constraint(path: &mut Path) {
    for path in path.segments.iter_mut() {
        if let PathArguments::AngleBracketed(generics) = &mut path.arguments {
            let new_generic_args = generics
                .args
                .clone()
                .into_iter()
                .filter_map(|mut arg| {
                    match &mut arg {
                        GenericArgument::AssocType(assoc) => {
                            if let Type::Path(path) = &assoc.ty {
                                if let Some(segment) = path.path.segments.first() {
                                    if segment.ident == Ident::new("Self", Span::call_site()) {
                                        return None;
                                    }
                                }
                            }
                        }
                        GenericArgument::Constraint(constraint) => {
                            for bound in constraint.bounds.iter_mut() {
                                if let TypeParamBound::Trait(trait_bound) = bound {
                                    filter_assoc_self_constraint(&mut trait_bound.path);
                                }
                            }
                        }
                        _ => {}
                    }
                    Some(arg)
                })
                .collect();

            generics.args = new_generic_args;
        }
    }
}
