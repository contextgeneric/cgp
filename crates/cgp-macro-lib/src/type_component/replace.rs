// pub fn replace_self_assoc_type

use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::visit_mut::{VisitMut, visit_type_mut, visit_type_param_bound_mut};
use syn::{Ident, TraitItemType, Type, TypeParamBound, parse_quote};

pub fn get_bounds_and_replace_self_assoc_type(
    item_type: &TraitItemType,
) -> Punctuated<TypeParamBound, Plus> {
    let mut bounds = item_type.bounds.clone();
    let mut visitor = ReplaceSelfAssocTypeVisitor {
        type_ident: item_type.ident.clone(),
    };

    for bound in &mut bounds {
        visit_type_param_bound_mut(&mut visitor, bound);
    }

    bounds
}

pub struct ReplaceSelfAssocTypeVisitor {
    pub type_ident: Ident,
}

impl VisitMut for ReplaceSelfAssocTypeVisitor {
    fn visit_type_mut(&mut self, node: &mut Type) {
        if let Type::Path(type_path) = node {
            let type_ident = &self.type_ident;
            if type_path == &parse_quote! { Self :: #type_ident } {
                *node = parse_quote!(#type_ident);
                return;
            }
        }

        visit_type_mut(self, node);
    }
}
