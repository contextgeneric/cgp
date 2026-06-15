use proc_macro2::TokenStream;
use quote::quote;
use syn::token::Mut;
use syn::{Ident, ImplItemFn, Type, parse2};

use crate::types::cgp_getter::GetterField;
use crate::types::getter::{ContextArg, FieldMode};

pub fn derive_getter_method(
    context_arg: &ContextArg,
    getter_field: &GetterField,
    tag_type: &Type,
    provider_ident: Option<Ident>,
) -> syn::Result<ImplItemFn> {
    GetterMethod {
        context_arg: context_arg.clone(),
        getter_field: getter_field.clone(),
        tag_type: tag_type.clone(),
        provider_ident,
    }
    .to_item_fn()
}

pub struct GetterMethod {
    pub context_arg: ContextArg,
    pub getter_field: GetterField,
    pub tag_type: Type,
    pub provider_ident: Option<Ident>,
}

impl GetterMethod {
    pub fn to_item_fn(&self) -> syn::Result<ImplItemFn> {
        let Self {
            context_arg,
            getter_field,
            tag_type,
            provider_ident,
        } = self;

        let getter_ident = &getter_field.field_name;

        let phantom_arg = match &getter_field.phantom_arg_type {
            Some(phantom) => {
                quote! {
                    , _phantom: PhantomData< #phantom >
                }
            }
            None => TokenStream::new(),
        };

        let context_fn_arg = match &context_arg {
            ContextArg::SelfArg => {
                if getter_field.receiver_mut.is_none() {
                    quote! { &self }
                } else {
                    quote! { &mut self }
                }
            }
            ContextArg::Type(context_type) => {
                if getter_field.receiver_mut.is_none() {
                    quote! { __context__: & #context_type}
                } else {
                    quote! { __context__: &mut #context_type }
                }
            }
        };

        let get_field_method = if getter_field.receiver_mut.is_none() {
            quote! { get_field }
        } else {
            quote! { get_field_mut }
        };

        let context_var = match &context_arg {
            ContextArg::SelfArg => {
                quote! { self }
            }
            ContextArg::Type(_) => {
                quote! { __context__ }
            }
        };

        let call_expr = match provider_ident {
            Some(provider_ident) => {
                quote! {
                    #provider_ident :: #get_field_method ( #context_var, ::core::marker::PhantomData::< #tag_type > )
                }
            }
            None => {
                quote! {
                    #context_var . #get_field_method ( ::core::marker::PhantomData::< #tag_type > )
                }
            }
        };

        let call_expr = extend_call_expr(
            call_expr,
            &getter_field.field_mode,
            &getter_field.receiver_mut,
        );

        let return_type = &getter_field.return_type;

        parse2(quote! {
            fn #getter_ident( #context_fn_arg #phantom_arg ) -> #return_type {
                #call_expr
            }
        })
    }
}

fn extend_call_expr(
    call_expr: TokenStream,
    field_mode: &FieldMode,
    field_mut: &Option<Mut>,
) -> TokenStream {
    match field_mode {
        FieldMode::Reference => call_expr,
        FieldMode::OptionRef => {
            if field_mut.is_none() {
                quote! {
                    #call_expr .as_ref()
                }
            } else {
                quote! {
                    #call_expr .as_mut()
                }
            }
        }
        FieldMode::MRef => {
            quote! {
                MRef::Ref( #call_expr )
            }
        }
        FieldMode::Str => {
            if field_mut.is_none() {
                quote! {
                    #call_expr .as_str()
                }
            } else {
                quote! {
                    #call_expr .as_mut_str()
                }
            }
        }
        FieldMode::Copy => {
            quote! {
                #call_expr .clone()
            }
        }
        FieldMode::Slice => {
            quote! {
                #call_expr .as_ref()
            }
        }
    }
}
