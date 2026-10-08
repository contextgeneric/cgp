use syn::visit::{self, Visit};
use syn::{Type, TypeImplTrait};

/// The first `impl Trait` type inside `ty`, at any depth.
///
/// A handler function's parameter and return types become a provider impl's
/// trait arguments and `Output` associated type, where `impl Trait` is not
/// allowed (`E0562`), so the function macros reject it up front.
pub fn find_impl_trait(ty: &Type) -> Option<&TypeImplTrait> {
    let mut finder = FindImplTrait { found: None };
    finder.visit_type(ty);
    finder.found
}

struct FindImplTrait<'ast> {
    found: Option<&'ast TypeImplTrait>,
}

impl<'ast> Visit<'ast> for FindImplTrait<'ast> {
    fn visit_type_impl_trait(&mut self, node: &'ast TypeImplTrait) {
        if self.found.is_none() {
            self.found = Some(node);
        }
    }

    fn visit_type(&mut self, node: &'ast Type) {
        if self.found.is_none() {
            visit::visit_type(self, node);
        }
    }
}
