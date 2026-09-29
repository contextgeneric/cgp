use syn::visit::Visit;
use syn::{Ident, Type, visit};

use crate::visitors::bare_alias_ident;

/// Collect every **bare alias reference** a type contains, in source order.
///
/// This is the read-only counterpart of
/// [`SubstituteAbstractTypes`](crate::visitors::SubstituteAbstractTypes): where
/// that visitor *rewrites* each reference, this one only reports where they are,
/// which is what the `#[use_type]` grounding-cycle check needs to read one spec's
/// dependencies on another. Both decide what counts as a reference through the
/// shared [`bare_alias_ident`], so a position the substitution would rewrite is
/// always a position the cycle check can see.
///
/// The collected identifiers borrow from the traversed type, so each one carries
/// the span of the token the user wrote — which is what lets a rejected cycle put
/// its caret on the offending alias rather than on the whole macro block.
#[derive(Default)]
pub struct CollectBareAliases<'ast> {
    pub idents: Vec<&'ast Ident>,
}

impl<'ast> Visit<'ast> for CollectBareAliases<'ast> {
    fn visit_type(&mut self, ty: &'ast Type) {
        if let Some(ident) = bare_alias_ident(ty) {
            self.idents.push(ident);
            // A bare alias is a single argument-free segment, so it has no nested
            // types to descend into.
            return;
        }

        visit::visit_type(self, ty);
    }
}

/// Every bare alias reference in `ty`, in source order.
pub fn collect_bare_aliases(ty: &Type) -> Vec<&Ident> {
    let mut visitor = CollectBareAliases::default();
    visitor.visit_type(ty);
    visitor.idents
}
