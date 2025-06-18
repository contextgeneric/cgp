use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Colon, Comma};
use syn::{
    parse2, AngleBracketedGenericArguments, FieldValue, GenericArgument, GenericParam, Generics,
    Ident, ItemImpl, ItemStruct, LitInt, Member, Type, TypeParam,
};

use crate::symbol::symbol_from_string;

pub fn derive_builder(body: TokenStream) -> syn::Result<TokenStream> {
    let context_struct: ItemStruct = parse2(body)?;

    let context_ident = &context_struct.ident;
    let builder_ident = Ident::new(&format!("Partial{}", context_ident), context_ident.span());

    let builder_struct = derive_builder_struct(&context_struct, &builder_ident)?;

    let has_builder_impl = derive_has_builder_impl(&context_struct, &builder_ident)?;

    let build_field_impls = derive_build_field_impls(&context_struct, &builder_ident)?;

    let finalize_build_impl = derive_finalize_build_impl(&context_struct, &builder_ident)?;

    let mut out = quote! {
        #builder_struct

        #has_builder_impl

        #finalize_build_impl
    };

    out.append_all(build_field_impls);

    Ok(out)
}

pub fn derive_builder_struct(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<ItemStruct> {
    let mut builder_struct = context_struct.clone();
    builder_struct.ident = builder_ident.clone();

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

pub fn derive_build_field_impls(
    context_struct: &ItemStruct,
    builder_ident: &Ident,
) -> syn::Result<Vec<ItemImpl>> {
    let mut item_impls = Vec::new();

    let base_generic_args: AngleBracketedGenericArguments =
        if context_struct.generics.params.is_empty() {
            parse2(quote! { < > })?
        } else {
            parse2(context_struct.generics.split_for_impl().1.to_token_stream())?
        };

    for (current_index, current_field) in context_struct.fields.iter().enumerate() {
        let value_type = &current_field.ty;

        let mut generics = context_struct.generics.clone();
        let mut source_generic_args = base_generic_args.args.clone();
        let mut output_generic_args = base_generic_args.args.clone();
        let mut builder_fields = <Punctuated<FieldValue, Comma>>::new();

        for (other_index, other_field) in context_struct.fields.iter().enumerate() {
            let field_member = match &other_field.ident {
                Some(ident) => Member::Named(ident.clone()),
                None => Member::Unnamed(other_index.into()),
            };

            if other_index != current_index {
                let generic_param_name =
                    Ident::new(&format!("__F{}__", other_index), Span::call_site());
                generics.params.push(parse2(quote! {
                    #generic_param_name: MapType
                })?);

                let generic_arg: GenericArgument = parse2(quote! { #generic_param_name })?;
                source_generic_args.push(generic_arg.clone());
                output_generic_args.push(generic_arg);

                builder_fields.push(FieldValue {
                    attrs: Vec::new(),
                    member: field_member.clone(),
                    colon_token: Some(Colon(Span::call_site())),
                    expr: parse2(quote! { self. #field_member })?,
                });
            } else {
                source_generic_args.push(parse2(quote! { IsNothing })?);
                output_generic_args.push(parse2(quote! { IsPresent })?);

                builder_fields.push(FieldValue {
                    attrs: Vec::new(),
                    member: field_member.clone(),
                    colon_token: Some(Colon(Span::call_site())),
                    expr: parse2(quote! { value })?,
                });
            }
        }

        let source_type: Type = parse2(quote! {
            #builder_ident < #source_generic_args >
        })?;

        let output_type: Type = parse2(quote! {
            #builder_ident < #output_generic_args >
        })?;

        let tag_type = match &current_field.ident {
            Some(ident) => symbol_from_string(&ident.to_string()),
            None => {
                let index = LitInt::new(&format!("{current_index}"), current_field.span());

                parse2(quote! { Index< #index > })?
            }
        };

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let item_impl = parse2(quote! {
            impl #impl_generics BuildField< #tag_type >
                for #source_type
            #where_clause
            {
                type Value = #value_type;

                type Output = #output_type;

                fn build_field(self, _tag: ::core::marker::PhantomData< #tag_type >, value: Self::Value) -> Self::Output {
                    #builder_ident {
                        #builder_fields
                    }
                }
            }
        })?;

        item_impls.push(item_impl);
    }

    Ok(item_impls)
}

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
