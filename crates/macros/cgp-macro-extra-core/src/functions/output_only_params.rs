use syn::visit::{self, Visit};
use syn::{Error, FnArg, GenericParam, Ident, Signature, WherePredicate};

/// Reject a type or const parameter that no argument type names and no other bound names.
///
/// The generated `Computer` impl carries the function's generics, but its trait arguments carry
/// only the input types, so a parameter only the return type uses is constrained by nothing and
/// the impl fails with `E0207`. A parameter another bound names may still be constrained (an
/// associated-type bound such as `I: Iterator<Item = T>` binds `T`), so it is left to the compiler.
pub fn reject_output_only_params(sig: &Signature) -> syn::Result<()> {
    for param in sig.generics.params.iter() {
        let ident = match param {
            GenericParam::Type(param) => &param.ident,
            GenericParam::Const(param) => &param.ident,
            GenericParam::Lifetime(_) => continue,
        };

        if named_by_inputs(sig, ident) || named_by_other_bounds(sig, ident) {
            continue;
        }

        return Err(Error::new_spanned(
            ident,
            format!(
                "the parameter `{ident}` appears in no argument type, so the generated `Computer` impl cannot determine it; take an argument whose type names `{ident}`, or write the provider by hand as a `Computer` impl on a provider generic over `{ident}`"
            ),
        ));
    }

    Ok(())
}

fn named_by_inputs(sig: &Signature, ident: &Ident) -> bool {
    let mut finder = FindIdent::new(ident);

    for input in sig.inputs.iter() {
        if let FnArg::Typed(arg) = input {
            finder.visit_type(&arg.ty);
        }
    }

    finder.found
}

/// Whether a bound other than the parameter's own names it: an inline bound of another parameter,
/// or a `where` predicate whose bounded type is not the parameter alone.
fn named_by_other_bounds(sig: &Signature, ident: &Ident) -> bool {
    let mut finder = FindIdent::new(ident);

    for param in sig.generics.params.iter() {
        match param {
            GenericParam::Type(other) if other.ident != *ident => {
                for bound in other.bounds.iter() {
                    finder.visit_type_param_bound(bound);
                }
            }
            GenericParam::Const(other) if other.ident != *ident => {
                finder.visit_type(&other.ty);
            }
            _ => {}
        }
    }

    if let Some(where_clause) = &sig.generics.where_clause {
        for predicate in where_clause.predicates.iter() {
            if let WherePredicate::Type(predicate) = predicate {
                let bounds_itself = matches!(
                    &predicate.bounded_ty,
                    syn::Type::Path(path) if path.qself.is_none() && path.path.is_ident(ident)
                );

                if !bounds_itself {
                    finder.visit_predicate_type(predicate);
                }
            }
        }
    }

    finder.found
}

struct FindIdent<'a> {
    target: &'a Ident,
    found: bool,
}

impl<'a> FindIdent<'a> {
    fn new(target: &'a Ident) -> Self {
        Self {
            target,
            found: false,
        }
    }
}

impl<'ast> Visit<'ast> for FindIdent<'_> {
    fn visit_ident(&mut self, ident: &'ast Ident) {
        if ident == self.target {
            self.found = true;
        }

        visit::visit_ident(self, ident);
    }
}
