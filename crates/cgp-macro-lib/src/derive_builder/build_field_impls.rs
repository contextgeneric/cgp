use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Colon, Comma};
use syn::{
    parse2, AngleBracketedGenericArguments, FieldValue, GenericArgument, Ident, ItemImpl,
    ItemStruct, LitInt, Member, Type,
};

use crate::symbol::symbol_from_string;

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
