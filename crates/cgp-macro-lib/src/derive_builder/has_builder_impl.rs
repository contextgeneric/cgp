use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FieldValue, Ident, ItemImpl, ItemStruct, parse2};

use crate::derive_builder::{field_to_member, field_value_expr, to_generic_args};

pub fn derive_has_builder_impl(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let (impl_generics, ty_generics, where_clause) = context_struct.generics.split_for_impl();

    let context_ident = &context_struct.ident;

    let mut builder_generics = to_generic_args(&context_struct.generics)?;

    let mut builder_fields = <Punctuated<FieldValue, Comma>>::new();

    for (i, field) in context_struct.fields.iter().enumerate() {
        builder_generics.args.push(parse2(quote! {
            IsNothing
        })?);

        let field_member = field_to_member(i, field);

        builder_fields.push(field_value_expr(field_member, quote! { () })?);
    }

    let item_impl = parse2(quote! {
        impl #impl_generics HasBuilder
            for #context_ident #ty_generics
        #where_clause
        {
            type Builder = #builder_ident #builder_generics;

            fn builder() -> Self::Builder {
                #builder_ident {
                    #builder_fields
                }
            }
        }
    })?;

    Ok(item_impl)
}
