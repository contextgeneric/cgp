use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Comma};
use syn::{
    parse2, AngleBracketedGenericArguments, FieldValue, Ident, ItemImpl, ItemStruct, Member, Type,
};

pub fn derive_finalize_build_impl(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let context_ident = &context_struct.ident;
    let generics = &context_struct.generics;

    let mut generic_args: AngleBracketedGenericArguments = if generics.params.is_empty() {
        parse2(quote! { < > })?
    } else {
        parse2(generics.split_for_impl().1.to_token_stream())?
    };

    let mut builder_fields = <Punctuated<FieldValue, Comma>>::new();

    for (i, field) in context_struct.fields.iter().enumerate() {
        generic_args.args.push(parse2(quote! {
            IsPresent
        })?);

        let field_member = match &field.ident {
            Some(ident) => Member::Named(ident.clone()),
            None => Member::Unnamed(i.into()),
        };

        builder_fields.push(FieldValue {
            attrs: Vec::new(),
            member: field_member.clone(),
            colon_token: Some(Colon(Span::call_site())),
            expr: parse2(quote! { self. #field_member })?,
        });
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let builder_type: Type = parse2(quote! {
        #builder_ident #generic_args
    })?;

    let context_type: Type = parse2(quote! {
        #context_ident #ty_generics
    })?;

    let item_impl = parse2(quote! {
        impl #impl_generics FinalizeBuild for #builder_type
        #where_clause
        {
            type Output = #context_type;

            fn finalize_build(self) -> Self::Output {
                #context_ident {
                    #builder_fields
                }
            }
        }
    })?;

    Ok(item_impl)
}
