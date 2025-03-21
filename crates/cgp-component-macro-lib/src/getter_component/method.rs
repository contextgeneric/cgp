use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::getter_component::GetterField;

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
    let provider_type = &spec.provider_type;

    let phantom_arg = match &spec.phantom {
        Some(phantom) => {
            quote! {
                , _phantom: #phantom
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

    quote! {
        fn #field_name( #context_fn_arg #phantom_arg ) -> & #provider_type {
            #call_expr
        }
    }
}
