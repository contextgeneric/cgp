use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Comma};
use syn::{
    parse2, FieldValue, GenericParam, Generics, Ident, ItemImpl, ItemStruct, Member, Type,
    TypeParam,
};

pub fn derive_builder(body: TokenStream) -> syn::Result<TokenStream> {
    let context_struct: ItemStruct = parse2(body)?;

    let builder_struct = derive_builder_struct(&context_struct)?;

    let has_builder_impl = derive_has_builder_impl(&context_struct, &builder_struct)?;

    Ok(quote! {
        #builder_struct

        #has_builder_impl
    })
}

pub fn derive_builder_struct(context_struct: &ItemStruct) -> syn::Result<ItemStruct> {
    let context_name = &context_struct.ident;

    let mut builder_struct = context_struct.clone();

    let builder_name = Ident::new(&format!("Partial{}", context_name), context_name.span());
    builder_struct.ident = builder_name;

    let generics = &mut builder_struct.generics;

    for (i, field) in builder_struct.fields.iter_mut().enumerate() {
        let generic_param_name = Ident::new(&format!("__F{}__", i), Span::call_site());

        let generic_param: TypeParam = parse2(quote! {
            #generic_param_name : MapType
        })?;

        generics.params.push(GenericParam::Type(generic_param));

        let field_type = &field.ty;

        let mapped_type: Type = parse2(quote! {
            <#generic_param_name as MapType>::Mapped<#field_type>
        })?;

        field.ty = mapped_type;
    }

    Ok(builder_struct)
}

pub fn derive_has_builder_impl(
    context_struct: &ItemStruct,
    builder_struct: &ItemStruct,
) -> syn::Result<ItemImpl> {
    let (impl_generics, ty_generics, where_clause) = context_struct.generics.split_for_impl();

    let context_ident = &context_struct.ident;
    let builder_ident = &builder_struct.ident;

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
