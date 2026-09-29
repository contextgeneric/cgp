use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{Generics, Type, TypeParamBound, parse_quote};

pub trait ToTypeParamBounds {
    fn to_type_param_bounds(&self) -> syn::Result<Punctuated<TypeParamBound, Plus>>;
}

pub trait AddTypeParamBounds {
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
