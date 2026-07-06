use syn::visit_mut::VisitMut;
use syn::{PathArguments, Type, TypePath, parse_quote, visit_mut};

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
}
