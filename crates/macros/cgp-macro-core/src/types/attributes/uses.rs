use syn::TypeParamBound;
use syn::punctuated::Punctuated;
use syn::token::Plus;

use crate::parse_internal;
use crate::traits::ToTypeParamBounds;
use crate::types::ident::IdentWithTypeArgs;

#[derive(Default)]
pub struct UsesAttributes {
    pub imports: Vec<IdentWithTypeArgs>,
}

impl ToTypeParamBounds for UsesAttributes {
    fn to_type_param_bounds(&self) -> syn::Result<Punctuated<TypeParamBound, Plus>> {
        let mut bounds: Punctuated<TypeParamBound, Plus> = Punctuated::default();

        for import in &self.imports {
            bounds.push(parse_internal! { #import });
        }

        Ok(bounds)
    }
}
