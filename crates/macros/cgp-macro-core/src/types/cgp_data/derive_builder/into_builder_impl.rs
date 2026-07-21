use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FieldValue, Ident, ItemImpl, ItemStruct};

use crate::exports::{IntoBuilder, IsPresent};
use crate::functions::override_item_span;
use crate::parse_internal;
use crate::types::cgp_data::{field_to_member, field_value_expr, to_generic_args};

pub fn derive_into_builder_impl(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let (impl_generics, ty_generics, where_clause) = context_struct.generics.split_for_impl();

    let context_ident = &context_struct.ident;

    let mut builder_generics = to_generic_args(&context_struct.generics)?;

    let mut builder_fields = <Punctuated<FieldValue, Comma>>::new();

    for (i, field) in context_struct.fields.iter().enumerate() {
        builder_generics.args.push(parse_internal! {
            #IsPresent
        });

        let field_member = field_to_member(i, field);

        builder_fields.push(field_value_expr(
            field_member.clone(),
            quote! { self. #field_member },
        )?);
    }

    let item_impl = parse_internal! {
        impl #impl_generics #IntoBuilder
            for #context_ident #ty_generics
        #where_clause
        {
            type Builder = #builder_ident #builder_generics;

            fn into_builder(self) -> Self::Builder {
                #builder_ident {
                    #builder_fields
                }
            }
        }
    };

    // Key the error span on the struct name the user wrote, not the whole
    // derive. See docs/implementation/README.md#spans.
    override_item_span(context_ident.span(), &item_impl)
}
