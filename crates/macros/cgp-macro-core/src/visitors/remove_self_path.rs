use syn::visit_mut::{VisitMut, visit_type_mut};
use syn::{Ident, PathArguments, Type, TypePath};

pub struct RemoveSelfPathVisitor<'a> {
    pub assoc_idents: &'a [Ident],
}

impl VisitMut for RemoveSelfPathVisitor<'_> {
    fn visit_type_mut(&mut self, node: &mut Type) {
        if let Type::Path(TypePath { qself: None, path }) = node
            && path.leading_colon.is_none()
            && path.segments.len() >= 2
            && path.segments[0].ident == "Self"
            && matches!(path.segments[0].arguments, PathArguments::None)
            && self.assoc_idents.contains(&path.segments[1].ident)
        {
            let segments = std::mem::take(&mut path.segments);
            path.segments = segments.into_iter().skip(1).collect();
        }

        visit_type_mut(self, node);
    }
}
