use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::derive_getter::{FieldMode, GetterField};

pub enum ContextArg {
    SelfArg,
    Ident(Ident),
}

pub fn derive_getter_method(
    context_arg: &ContextArg,
    spec: &GetterField,
    phantom_generics: Option<TokenStream>,
    provider_ident: Option<Ident>,
) -> TokenStream {
    let field_name = &spec.field_name;

    let phantom_arg = match &spec.phantom_arg_type {
        Some(phantom) => {
            quote! {
                , _phantom: PhantomData< #phantom >
            }
        }
        None => TokenStream::new(),
    };

    let context_fn_arg = match &context_arg {
        ContextArg::SelfArg => {
            if spec.field_mut.is_none() {
                quote! { &self }
            } else {
                quote! { &mut self }
            }
        }
        ContextArg::Ident(context_type) => {
            if spec.field_mut.is_none() {
                quote! { context: & #context_type}
            } else {
                quote! { context: &mut #context_type }
            }
        }
    };

    let get_field_method = if spec.field_mut.is_none() {
        quote! { get_field }
    } else {
        quote! { get_field_mut }
    };

    let context_var = match &context_arg {
        ContextArg::SelfArg => {
            quote! { self }
        }
        ContextArg::Ident(_) => {
            quote! { context }
        }
    };

    let call_expr = match provider_ident {
        Some(provider_ident) => {
            quote! {
                #provider_ident :: #get_field_method ( #context_var, ::core::marker::PhantomData #phantom_generics )
            }
        }
        None => {
            quote! {
                #context_var . #get_field_method ( ::core::marker::PhantomData #phantom_generics )
            }
        }
    };

    let call_expr = match spec.field_mode {
        FieldMode::Reference => call_expr,
        FieldMode::OptionRef => {
            if spec.field_mut.is_none() {
                quote! {
                    #call_expr .as_ref()
                }
            } else {
                quote! {
                    #call_expr .as_mut()
                }
            }
        }
        FieldMode::Str => {
            if spec.field_mut.is_none() {
                quote! {
                    #call_expr .as_str()
                }
            } else {
                quote! {
                    #call_expr .as_mut_str()
                }
            }
        }
        FieldMode::Clone => quote! {
            #call_expr .clone()
        },
    };

    let return_type = &spec.return_type;

    quote! {
        fn #field_name( #context_fn_arg #phantom_arg ) -> #return_type {
            #call_expr
        }
    }
}
