use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;
use syn::{ExprPath, PathArguments, PathSegment, Token, Type, TypePath, parse_quote, visit_mut};

use crate::types::attributes::UseTypeAttribute;

/// A single-pass `VisitMut` that rewrites every bare, single-segment,
/// argument-free type path matching an imported alias into its fully-qualified
/// `<Context as Trait>::AssocType` form.
///
/// Unlike a per-spec visitor, this holds *every* `#[use_type]` spec at once, so
/// one traversal of the item handles all imports regardless of the order they
/// were written. Because the imported aliases are guaranteed unique (see
/// `forbid_duplicate_aliases`), at most one spec can match a given identifier,
/// so the match order among specs is irrelevant.
///
/// Each spec's `context_type` must already be *grounded* — resolved to a fully
/// qualified path (`<Self as HasTypes>::Types`) with no remaining bare alias —
/// before the visitor runs. Grounding is what lets a single traversal suffice:
/// the replacement a spec emits contains no bare alias, so the visitor never has
/// to revisit its own output to finish a nested import.
///
/// `is_changed` records whether any replacement was made during the traversal,
/// which the grounding fixpoint reads to decide when a further pass would be a
/// no-op.
pub struct SubstituteAbstractTypes<'a> {
    pub specs: &'a [UseTypeAttribute],
    pub is_changed: bool,
}

impl<'a> SubstituteAbstractTypes<'a> {
    pub fn new(specs: &'a [UseTypeAttribute]) -> Self {
        Self {
            specs,
            is_changed: false,
        }
    }
}

impl VisitMut for SubstituteAbstractTypes<'_> {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        if let Type::Path(TypePath { qself: None, path }) = ty
            && path.leading_colon.is_none()
            && path.segments.len() == 1
        {
            let segment = &path.segments[0];
            if matches!(segment.arguments, PathArguments::None) {
                for spec in self.specs {
                    if let Some(replacement_ident) = spec.replace_ident(&segment.ident) {
                        let trait_path = &spec.trait_path;
                        let context_type = &spec.context_type;
                        *ty = parse_quote! { <#context_type as #trait_path>::#replacement_ident };
                        self.is_changed = true;
                        return;
                    }
                }
            }
        }
        visit_mut::visit_type_mut(self, ty);
    }

    /// Rewrite an alias that *qualifies* an expression path — `Transaction::begin_from(pool)` —
    /// into the qualified-type form `<<Self as Trait>::Transaction>::begin_from(pool)`.
    ///
    /// The alias is already rewritten in every type position inside a body, a `let` annotation
    /// included, so leaving it unresolved as the qualifier of an associated-function or
    /// associated-const call was an inconsistency rather than a deliberate boundary: the author has
    /// claimed the name by importing it, and `forbid_duplicate_aliases` guarantees nothing else in
    /// the import list claims it too.
    ///
    /// Only a path of **two or more** segments is rewritten, which is what keeps the rewrite
    /// unambiguous. A multi-segment `Transaction::foo` can only mean an associated item of the type,
    /// while a bare single-segment `Transaction` in expression position is a *value* — a unit struct
    /// or an enum variant the author means — and an abstract type can never be one, so it is left
    /// alone.
    fn visit_expr_path_mut(&mut self, expr: &mut ExprPath) {
        if expr.qself.is_none()
            && expr.path.leading_colon.is_none()
            && expr.path.segments.len() > 1
            && matches!(expr.path.segments[0].arguments, PathArguments::None)
        {
            // Resolve against the specs before mutating, so the replacement is not computed while
            // the path it replaces is still borrowed.
            let replacement = self.specs.iter().find_map(|spec| {
                let replacement_ident = spec.replace_ident(&expr.path.segments[0].ident)?;
                let trait_path = &spec.trait_path;
                let context_type = &spec.context_type;
                Some(parse_quote! { <#context_type as #trait_path>::#replacement_ident })
            });

            if let Some(ty) = replacement {
                let ty: Type = ty;
                let rest: Punctuated<PathSegment, Token![::]> =
                    expr.path.segments.iter().skip(1).cloned().collect();

                *expr = parse_quote! { <#ty>::#rest };
                self.is_changed = true;
                return;
            }
        }
        visit_mut::visit_expr_path_mut(self, expr);
    }
}
