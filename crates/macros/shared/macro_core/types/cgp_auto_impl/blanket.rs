use syn::visit_mut::VisitMut;
use syn::{
    Error, Generics, Ident, ImplItem, ImplItemConst, ImplItemFn, ImplItemType, ItemImpl, ItemTrait,
    TraitItem, Type, Visibility,
};

use crate::macro_core::functions::{override_item_span, parse_internal};
use crate::macro_core::visitors::RemoveSelfPathVisitor;

/// Turn a trait's provided items into a blanket impl for `context_type`.
///
/// The returned trait is the input with method and const bodies removed; those
/// bodies are the impl. `impl_generics` is the impl header's generic list — the
/// caller inserts the context parameter (and any other params the impl already
/// has). Associated types on the trait are appended as further generic
/// parameters, and supertrait bounds are lowered onto the context.
///
/// `#[cgp_auto_impl]` calls this with a fresh `__Context__`. `#[cgp_impl]` helper
/// methods call it with the provider impl's existing generics, so later macros
/// share one blanket-impl base.
pub fn derive_blanket_items(
    context_type: &Type,
    item_trait: &ItemTrait,
    mut impl_generics: Generics,
) -> syn::Result<(ItemTrait, ItemImpl)> {
    let emitted_trait = strip_provided_bodies(item_trait)?;

    let mut source = item_trait.clone();

    let assoc_idents = collect_assoc_idents(&source);

    let mut remove_self = RemoveSelfPathVisitor {
        assoc_idents: &assoc_idents,
    };
    remove_self.visit_item_trait_mut(&mut source);
    remove_self.visit_generics_mut(&mut impl_generics);

    let mut impl_items: Vec<ImplItem> = Vec::new();

    for trait_item in source.items.iter_mut() {
        match trait_item {
            TraitItem::Type(trait_item_type) => {
                let item_type_ident = &trait_item_type.ident;

                impl_generics.params.push(parse_internal!(#item_type_ident));

                if !trait_item_type.bounds.is_empty() {
                    let bounds = trait_item_type.bounds.clone();
                    impl_generics
                        .make_where_clause()
                        .predicates
                        .push(parse_internal! {
                            #item_type_ident: #bounds
                        });
                }

                let type_impl: Type = parse_internal!(#item_type_ident);

                impl_items.push(ImplItem::Type(ImplItemType {
                    attrs: trait_item_type.attrs.clone(),
                    vis: Visibility::Inherited,
                    defaultness: None,
                    type_token: trait_item_type.type_token,
                    ident: trait_item_type.ident.clone(),
                    generics: trait_item_type.generics.clone(),
                    eq_token: Default::default(),
                    ty: type_impl,
                    semi_token: Default::default(),
                }));
            }
            TraitItem::Fn(trait_item_fn) => {
                let block = trait_item_fn.default.clone().ok_or_else(|| {
                    Error::new_spanned(
                        &trait_item_fn.sig,
                        "method must have a body to become the blanket provider",
                    )
                })?;

                impl_items.push(ImplItem::Fn(ImplItemFn {
                    attrs: trait_item_fn.attrs.clone(),
                    vis: Visibility::Inherited,
                    defaultness: None,
                    sig: trait_item_fn.sig.clone(),
                    block,
                }));
            }
            TraitItem::Const(trait_item_const) => {
                let const_ident = trait_item_const.ident.clone();
                let (eq_token, expr) = trait_item_const.default.clone().ok_or_else(|| {
                    Error::new_spanned(
                        const_ident,
                        "const item must have a value to become the blanket provider",
                    )
                })?;

                impl_items.push(ImplItem::Const(ImplItemConst {
                    attrs: trait_item_const.attrs.clone(),
                    vis: Visibility::Inherited,
                    defaultness: None,
                    const_token: trait_item_const.const_token,
                    ident: trait_item_const.ident.clone(),
                    generics: trait_item_const.generics.clone(),
                    colon_token: trait_item_const.colon_token,
                    ty: trait_item_const.ty.clone(),
                    eq_token,
                    expr,
                    semi_token: trait_item_const.semi_token,
                }));
            }
            _ => {
                return Err(Error::new_spanned(
                    trait_item,
                    "unsupported trait item in blanket impl",
                ));
            }
        }
    }

    if !source.supertraits.is_empty() {
        let supertraits = &source.supertraits;
        impl_generics
            .make_where_clause()
            .predicates
            .push(parse_internal! {
                #context_type: #supertraits
            });
    }

    let trait_name = &item_trait.ident;
    let (_, type_generics, _) = item_trait.generics.split_for_impl();
    let (impl_generics, _, where_clause) = impl_generics.split_for_impl();

    let mut item_impl: ItemImpl = parse_internal! {
        impl #impl_generics #trait_name #type_generics for #context_type
        #where_clause
        {
            #( #impl_items )*
        }
    };

    item_impl.unsafety = item_trait.unsafety;
    let item_impl = override_item_span(item_trait.ident.span(), &item_impl)?;

    Ok((emitted_trait, item_impl))
}

fn collect_assoc_idents(item_trait: &ItemTrait) -> Vec<Ident> {
    item_trait
        .items
        .iter()
        .filter_map(|item| {
            if let TraitItem::Type(assoc_type) = item {
                Some(assoc_type.ident.clone())
            } else {
                None
            }
        })
        .collect()
}

/// Clone the trait and drop provided method and const bodies, leaving the
/// signatures the blanket impl satisfies.
fn strip_provided_bodies(item_trait: &ItemTrait) -> syn::Result<ItemTrait> {
    let mut emitted = item_trait.clone();

    for item in &mut emitted.items {
        match item {
            TraitItem::Fn(func) => {
                if func.default.is_none() {
                    return Err(Error::new_spanned(
                        &func.sig,
                        "method must have a body to become the blanket provider",
                    ));
                }
                func.default = None;
                func.semi_token = Some(syn::token::Semi(func.sig.ident.span()));
            }
            TraitItem::Const(item_const) => {
                if item_const.default.is_none() {
                    return Err(Error::new_spanned(
                        item_const,
                        "const item must have a value to become the blanket provider",
                    ));
                }
                item_const.default = None;
            }
            TraitItem::Type(_) => {}
            _ => {
                return Err(Error::new_spanned(
                    item,
                    "unsupported trait item in blanket impl",
                ));
            }
        }
    }

    Ok(emitted)
}
