use syn::TypeParamBound;
use syn::punctuated::Punctuated;
use syn::token::Plus;

use crate::traits::ToTypeParamBounds;

/// The parsed `#[uses(...)]` attribute: the trait bounds a provider imports onto
/// `Self`. Each import is a full [`syn::TypeParamBound`], so an associated-type
/// binding (`HasErrorType<Error = anyhow::Error>`), an HRTB, or a lifetime bound
/// is accepted in addition to the plain `Trait<Params>` form.
/// The `#[uses]` imports of a function or impl block.
#[derive(Default)]
pub struct UsesAttributes {
    /// The imported bounds.
    pub imports: Vec<TypeParamBound>,
}

impl ToTypeParamBounds for UsesAttributes {
    fn to_type_param_bounds(&self) -> syn::Result<Punctuated<TypeParamBound, Plus>> {
        Ok(self.imports.iter().cloned().collect())
    }
}
