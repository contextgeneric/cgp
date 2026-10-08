use std::collections::BTreeSet;

use proc_macro2::Span;
use syn::visit::Visit;
use syn::visit_mut::{VisitMut, visit_type_reference_mut};
use syn::{
    Block, Error, FnArg, Ident, Item, ItemStruct, ItemTrait, Lifetime, Pat, TraitItem, TraitItemFn,
    Type, TypeReference, Visibility, WherePredicate,
};

use crate::macro_core::exports::CanLog;
use crate::macro_core::functions::{override_item_span, parse_internal, to_camel_case_str};
use crate::macro_core::types::cgp_auto_impl::derive_blanket_items;

/// `#[cgp_auto_log]` pipeline: log-method signatures become a blanket impl that
/// packs the arguments into a detail struct and calls `CanLog`.
pub struct ItemCgpAutoLog {
    pub item_trait: ItemTrait,
}

impl ItemCgpAutoLog {
    pub fn parse(item_trait: &ItemTrait) -> syn::Result<Self> {
        if !item_trait.generics.params.is_empty() {
            return Err(Error::new_spanned(
                &item_trait.generics,
                "#[cgp_auto_log] does not support generic traits",
            ));
        }

        if item_trait.items.is_empty() {
            return Err(Error::new_spanned(
                &item_trait.ident,
                "a logger trait must contain at least one method",
            ));
        }

        for item in &item_trait.items {
            if !matches!(item, TraitItem::Fn(_)) {
                return Err(Error::new_spanned(
                    item,
                    "a logger trait contains only methods",
                ));
            }
        }

        Ok(Self {
            item_trait: item_trait.clone(),
        })
    }

    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let mut source = self.item_trait.clone();
        let mut detail_structs: Vec<ItemStruct> = Vec::new();
        let mut bounds: Vec<WherePredicate> = Vec::new();
        let mut names = BTreeSet::new();

        for item in &mut source.items {
            let TraitItem::Fn(method) = item else {
                continue;
            };
            let (detail_struct, bound) = synthesize_log_method(method, &self.item_trait.vis)?;
            if !names.insert(detail_struct.ident.to_string()) {
                return Err(Error::new_spanned(
                    &method.sig.ident,
                    "this method's detail struct name collides with another log method",
                ));
            }
            detail_structs.push(detail_struct);
            bounds.push(bound);
        }

        let context_ident = Ident::new("__Context__", Span::call_site());
        let mut generics = source.generics.clone();
        generics.params.insert(0, parse_internal!(#context_ident));
        let context_type: Type = parse_internal!(#context_ident);

        let (emitted_trait, item_impl) = derive_blanket_items(&context_type, &source, generics)?;
        let mut item_impl = override_item_span(self.item_trait.ident.span(), &item_impl)?;

        let where_clause = item_impl.generics.make_where_clause();
        for bound in bounds {
            where_clause.predicates.push(bound);
        }

        let mut items = vec![emitted_trait.into()];
        items.extend(detail_structs.into_iter().map(Item::from));
        items.push(item_impl.into());
        Ok(items)
    }
}

fn synthesize_log_method(
    method: &mut TraitItemFn,
    vis: &Visibility,
) -> syn::Result<(ItemStruct, WherePredicate)> {
    if method.default.is_some() {
        return Err(Error::new_spanned(
            &method.sig,
            "remove the method body; #[cgp_auto_log] logs the arguments through CanLog",
        ));
    }
    if method.sig.asyncness.is_some() || method.sig.unsafety.is_some() {
        return Err(Error::new_spanned(
            &method.sig,
            "log methods must be synchronous safe functions",
        ));
    }
    if !method.sig.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &method.sig.generics,
            "#[cgp_auto_log] does not support generic methods",
        ));
    }

    let mut inputs = method.sig.inputs.iter();
    match inputs.next() {
        Some(FnArg::Receiver(receiver))
            if receiver.reference.is_some()
                && receiver.mutability.is_none()
                && receiver.colon_token.is_none() => {}
        Some(receiver) => {
            return Err(Error::new_spanned(
                receiver,
                "a log method takes `&self` so it can call `self.log`",
            ));
        }
        None => {
            return Err(Error::new_spanned(
                &method.sig,
                "a log method takes `&self` so it can call `self.log`",
            ));
        }
    }

    let mut fields: Vec<(Ident, Type)> = Vec::new();
    for arg in inputs {
        let FnArg::Typed(pat_type) = arg else {
            return Err(Error::new_spanned(
                arg,
                "only the first argument may be self",
            ));
        };
        let ident = match pat_type.pat.as_ref() {
            Pat::Ident(pat_ident) if pat_ident.by_ref.is_none() && pat_ident.subpat.is_none() => {
                pat_ident.ident.clone()
            }
            other => {
                return Err(Error::new_spanned(
                    other,
                    "log arguments must be plain identifiers",
                ));
            }
        };
        fields.push((ident, (*pat_type.ty).clone()));
    }

    let needs_lifetime = fields.iter().any(|(_, ty)| has_elided_lifetime(ty));
    let lifetime = Lifetime::new("'a", Span::call_site());
    if needs_lifetime {
        for (_, ty) in &mut fields {
            bind_elided_lifetime(ty, &lifetime);
        }
    }

    let detail_ident = Ident::new(
        &format!("__{}", to_camel_case_str(&method.sig.ident.to_string())),
        method.sig.ident.span(),
    );

    let field_vis_idents: Vec<&Ident> = fields.iter().map(|(ident, _)| ident).collect();
    let field_types: Vec<&Type> = fields.iter().map(|(_, ty)| ty).collect();

    let detail_struct: ItemStruct = if needs_lifetime {
        parse_internal! {
            #vis struct #detail_ident <#lifetime> {
                #( pub #field_vis_idents : #field_types, )*
            }
        }
    } else {
        parse_internal! {
            #vis struct #detail_ident {
                #( pub #field_vis_idents : #field_types, )*
            }
        }
    };

    let can_log = CanLog;
    let context = Ident::new("__Context__", Span::call_site());
    let bound: WherePredicate = if needs_lifetime {
        parse_internal! {
            #context: for <#lifetime> #can_log <#detail_ident <#lifetime>>
        }
    } else {
        parse_internal! {
            #context: #can_log <#detail_ident>
        }
    };

    let field_idents: Vec<&Ident> = fields.iter().map(|(ident, _)| ident).collect();
    let block: Block = parse_internal! {
        {
            self.log(#detail_ident { #( #field_idents, )* })
        }
    };
    method.default = Some(block);
    method.semi_token = None;

    Ok((detail_struct, bound))
}

fn has_elided_lifetime(ty: &Type) -> bool {
    struct FindElided {
        found: bool,
    }

    impl Visit<'_> for FindElided {
        fn visit_type_reference(&mut self, node: &TypeReference) {
            if node
                .lifetime
                .as_ref()
                .is_none_or(|lifetime| lifetime.ident == "_")
            {
                self.found = true;
            }
            syn::visit::visit_type_reference(self, node);
        }
    }

    let mut find = FindElided { found: false };
    find.visit_type(ty);
    find.found
}

fn bind_elided_lifetime(ty: &mut Type, lifetime: &Lifetime) {
    struct Bind<'a> {
        lifetime: &'a Lifetime,
    }

    impl VisitMut for Bind<'_> {
        fn visit_type_reference_mut(&mut self, node: &mut TypeReference) {
            if node
                .lifetime
                .as_ref()
                .is_none_or(|lifetime| lifetime.ident == "_")
            {
                node.lifetime = Some(self.lifetime.clone());
            }
            visit_type_reference_mut(self, node);
        }
    }

    Bind { lifetime }.visit_type_mut(ty);
}
