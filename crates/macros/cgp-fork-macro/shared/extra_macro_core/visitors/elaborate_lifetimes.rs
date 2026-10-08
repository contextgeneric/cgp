use proc_macro2::Span;
use syn::visit::Visit;
use syn::visit_mut::{self, VisitMut};
use syn::{Lifetime, ParenthesizedGenericArguments, Type, TypeBareFn, TypeReference};

/// Name every elided lifetime in a type: a reference written without one
/// (`&T`) and the placeholder `'_`, at any depth.
///
/// A dispatch method's argument and return types are copied into a `where`
/// bound and a helper function's signature, where an elided lifetime is either
/// rejected (`E0637`) or means something else, so each one must be given a name.
/// In [`fresh`](Self::fresh) mode every elided lifetime gets its own new name,
/// matching how the compiler treats elided input lifetimes, and the names are
/// recorded in `introduced`; in [`fixed`](Self::fixed) mode each one gets the
/// same given lifetime, matching how elided output lifetimes take the lifetime
/// of `&self`.
///
/// A function-pointer type (`fn(&T)`) and the `Fn(&T)` sugar bind their own
/// elided lifetimes, so the visitor leaves them alone.
pub struct ElaborateElidedLifetimes {
    fixed: Option<Lifetime>,
    next_index: usize,
    /// The lifetimes [`fresh`](Self::fresh) mode introduced, in order.
    pub introduced: Vec<Lifetime>,
}

impl ElaborateElidedLifetimes {
    /// Give each elided lifetime a new name, `'__a{n}__` counting from
    /// `first_index`.
    pub fn fresh(first_index: usize) -> Self {
        Self {
            fixed: None,
            next_index: first_index,
            introduced: Vec::new(),
        }
    }

    /// Give each elided lifetime the name `lifetime`.
    pub fn fixed(lifetime: Lifetime) -> Self {
        Self {
            fixed: Some(lifetime),
            next_index: 0,
            introduced: Vec::new(),
        }
    }

    fn next_lifetime(&mut self) -> Lifetime {
        match &self.fixed {
            Some(lifetime) => lifetime.clone(),
            None => {
                let lifetime =
                    Lifetime::new(&format!("'__a{}__", self.next_index), Span::call_site());
                self.next_index += 1;
                self.introduced.push(lifetime.clone());
                lifetime
            }
        }
    }
}

impl VisitMut for ElaborateElidedLifetimes {
    fn visit_type_reference_mut(&mut self, node: &mut TypeReference) {
        if node.lifetime.is_none() {
            node.lifetime = Some(self.next_lifetime());
        }

        visit_mut::visit_type_reference_mut(self, node);
    }

    fn visit_lifetime_mut(&mut self, node: &mut Lifetime) {
        if node.ident == "_" {
            *node = self.next_lifetime();
        }
    }

    fn visit_type_bare_fn_mut(&mut self, _node: &mut TypeBareFn) {}

    fn visit_parenthesized_generic_arguments_mut(
        &mut self,
        _node: &mut ParenthesizedGenericArguments,
    ) {
    }
}

/// The distinct lifetimes named in `types`, in order of first appearance,
/// skipping the ones a function-pointer type or `Fn(..)` sugar binds itself.
pub fn collect_lifetimes<'a>(types: impl IntoIterator<Item = &'a Type>) -> Vec<Lifetime> {
    let mut collector = CollectLifetimes::default();

    for ty in types {
        collector.visit_type(ty);
    }

    collector.lifetimes
}

#[derive(Default)]
struct CollectLifetimes {
    lifetimes: Vec<Lifetime>,
}

impl<'ast> Visit<'ast> for CollectLifetimes {
    fn visit_lifetime(&mut self, node: &'ast Lifetime) {
        if !self.lifetimes.contains(node) {
            self.lifetimes.push(node.clone());
        }
    }

    fn visit_type_bare_fn(&mut self, _node: &'ast TypeBareFn) {}

    fn visit_parenthesized_generic_arguments(
        &mut self,
        _node: &'ast ParenthesizedGenericArguments,
    ) {
    }
}
