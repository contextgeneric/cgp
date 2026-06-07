use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FieldValue, Ident, ItemImpl, ItemStruct, Type, parse2};

use crate::derive_builder::{field_to_member, field_value_expr, to_generic_args};

pub fn derive_finalize_build_impl(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let context_ident = &context_struct.ident;
    let generics = &context_struct.generics;

    let mut generic_args = to_generic_args(generics)?;

    let mut builder_fields = <Punctuated<FieldValue, Comma>>::new();

    for (i, field) in context_struct.fields.iter().enumerate() {
        generic_args.args.push(parse2(quote! {
            IsPresent
        })?);

        let field_member = field_to_member(i, field);

        builder_fields.push(field_value_expr(
            field_member.clone(),
            quote! { self. #field_member },
        )?);
    }

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let builder_type: Type = parse2(quote! {
        #builder_ident #generic_args
    })?;

    let item_impl = parse2(quote! {
        impl #impl_generics FinalizeBuild for #builder_type
        #where_clause
        {
            fn finalize_build(self) -> Self::Target {
                #context_ident {
                    #builder_fields
                }
            }
        }
    })?;

    Ok(item_impl)
}
