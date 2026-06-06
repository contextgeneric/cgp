use syn::visit_mut::VisitMut;
use syn::{PathArguments, Type, TypePath, parse_quote, visit_mut};

use crate::types::attributes::UseTypeAttribute;

pub struct SubstituteAbstractType<'a> {
    pub type_spec: &'a UseTypeAttribute,
}

impl VisitMut for SubstituteAbstractType<'_> {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        if let Type::Path(TypePath { qself: None, path }) = ty
            && path.leading_colon.is_none()
            && path.segments.len() == 1
        {
            let segment = &path.segments[0];
            if matches!(segment.arguments, PathArguments::None)
                && let Some(replacement_ident) = self.type_spec.replace_ident(&segment.ident)
            {
                let trait_path = &self.type_spec.trait_path;
                let context_type = &self.type_spec.context_type;
                *ty = parse_quote! { <#context_type as #trait_path>::#replacement_ident };
                return;
            }
        }
        visit_mut::visit_type_mut(self, ty);
    }
}
