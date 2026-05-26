use quote::quote;
use syn::{Generics, ItemImpl, Type, parse_quote, parse2};

use crate::exports::{DelegateComponent, IsProviderFor};
use crate::functions::merge_generics;

pub struct EvaluatedDelegateEntry {
    pub table_type: Type,
    pub generics: Generics,
    pub key: Type,
    pub value: Type,
}

pub trait EvalDelegateEntry {
    fn eval_entry(&self, table_type: &Type) -> syn::Result<EvaluatedDelegateEntry>;
}

pub trait EvalDelegateEntries {
    fn eval_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>>;
}

impl EvaluatedDelegateEntry {
    pub fn build_delegate_component_impl(
        &self,
        outer_generics: &Generics,
    ) -> syn::Result<ItemImpl> {
        let table_type = &self.table_type;

        let generics = merge_generics(outer_generics, &self.generics);

        let key = &self.key;
        let value = &self.value;

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        parse2(quote! {
            impl #impl_generics
                #DelegateComponent< #key >
                for #table_type
            #where_clause
            {
                type Delegate = #value;
            }
        })
    }

    pub fn build_is_provider_for_impl(&self, outer_generics: &Generics) -> syn::Result<ItemImpl> {
        let table_type = &self.table_type;

        let mut generics = merge_generics(outer_generics, &self.generics);

        let key = &self.key;
        let value = &self.value;

        generics.params.push(parse_quote!(__Context__));
        generics.params.push(parse_quote!(__Params__));

        generics.make_where_clause().predicates.push(parse_quote! {
            #value: #IsProviderFor<#key, __Context__, __Params__>
        });

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        parse2(quote! {
            impl #impl_generics
                #IsProviderFor< #key, __Context__, __Params__ >
                for #table_type
            #where_clause
            {}
        })
    }

    pub fn build_namespace_impl(
        &self,
        namespace_trait: &Type,
        outer_generics: &Generics,
    ) -> syn::Result<ItemImpl> {
        let generics = merge_generics(outer_generics, &self.generics);

        let key = &self.key;
        let value = &self.value;

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        parse2(quote! {
            impl #impl_generics
                #namespace_trait
                for #key
            #where_clause
            {
                type Delegate = #value;
            }
        })
    }
}
