use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{Generics, Type, TypeParamBound, parse_quote};

/// Converts a value into a list of type-parameter bounds.
pub trait ToTypeParamBounds {
    /// The bounds the value represents.
    fn to_type_param_bounds(&self) -> syn::Result<Punctuated<TypeParamBound, Plus>>;
}

/// Adds a value's bounds to a `where` clause for a self type.
pub trait AddTypeParamBounds {
    /// Adds the bounds to the generics.
    fn add_type_param_bounds(&self, self_type: &Type, generics: &mut Generics) -> syn::Result<()>;
}

impl<T> AddTypeParamBounds for T
where
    T: ToTypeParamBounds,
{
    fn add_type_param_bounds(&self, self_type: &Type, generics: &mut Generics) -> syn::Result<()> {
        let bounds = self.to_type_param_bounds()?;

        if !bounds.is_empty() {
            generics.make_where_clause().predicates.push(parse_quote! {
                #self_type: #bounds
            });
        }

        Ok(())
    }
}
