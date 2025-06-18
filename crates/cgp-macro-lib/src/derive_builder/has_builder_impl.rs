use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Comma};
use syn::{parse2, FieldValue, Generics, Ident, ItemImpl, ItemStruct, Member};

pub fn derive_has_builder_impl(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let (impl_generics, ty_generics, where_clause) = context_struct.generics.split_for_impl();

    let context_ident = &context_struct.ident;

    let mut builder_generics = parse2::<Generics>(ty_generics.to_token_stream())?.params;

    let mut builder_fields = <Punctuated<FieldValue, Comma>>::new();

    for (i, field) in context_struct.fields.iter().enumerate() {
        builder_generics.push(parse2(quote! {
            IsNothing
        })?);

        let field_member = match &field.ident {
            Some(ident) => Member::Named(ident.clone()),
            None => Member::Unnamed(i.into()),
        };

        builder_fields.push(FieldValue {
            attrs: Vec::new(),
            member: field_member,
            colon_token: Some(Colon(Span::call_site())),
            expr: parse2(quote! { () })?,
        });
    }

    let item_impl = parse2(quote! {
        impl #impl_generics HasBuilder
            for #context_ident #ty_generics
        #where_clause
        {
            type Builder = #builder_ident < #builder_generics >;

            fn builder() -> Self::Builder {
                #builder_ident {
                    #builder_fields
                }
            }
        }
    })?;

    Ok(item_impl)
}
