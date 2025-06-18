use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Colon;
use syn::{
    parse2, AngleBracketedGenericArguments, Field, FieldMutability, FieldValue, Fields,
    FieldsUnnamed, Generics, Ident, Member, Type, Variant, Visibility,
};

pub fn to_generic_args(generics: &Generics) -> syn::Result<AngleBracketedGenericArguments> {
    if generics.params.is_empty() {
        parse2(quote! { < > })
    } else {
        parse2(generics.split_for_impl().1.to_token_stream())
    }
}

pub fn field_to_member(index: usize, field: &Field) -> Member {
    match &field.ident {
        Some(ident) => Member::Named(ident.clone()),
        None => Member::Unnamed(index.into()),
    }
}

pub fn index_to_generic_ident(index: usize) -> Ident {
    Ident::new(&format!("__F{}__", index), Span::call_site())
}

pub fn field_value_expr(field_member: Member, expr: TokenStream) -> syn::Result<FieldValue> {
    Ok(FieldValue {
        attrs: Vec::new(),
        member: field_member,
        colon_token: Some(Colon(Span::call_site())),
        expr: parse2(expr)?,
    })
}

pub fn get_variant_type(variant: &Variant) -> syn::Result<&Type> {
    match &variant.fields {
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            if let Some(field) = fields.unnamed.first() {
                return Ok(&field.ty);
            }
        }
        _ => {}
    }

    return Err(syn::Error::new(
        variant.span(),
        "Expected variant to contain exactly one unnamed field",
    ));
}

pub fn type_to_variant_fields(type_: &Type) -> Fields {
    Fields::Unnamed(FieldsUnnamed {
        unnamed: Punctuated::from_iter([Field {
            attrs: Vec::new(),
            ident: None,
            vis: Visibility::Inherited,
            ty: type_.clone(),
            colon_token: None,
            mutability: FieldMutability::None,
        }]),
        paren_token: Default::default(),
    })
}
