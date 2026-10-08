use proc_macro2::Span;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::token::{Eq, Type as TypeToken};
use syn::{Ident, Type};

use crate::macro_core::exports::{UseField, UseType};
use crate::macro_core::functions::{parse_internal, to_camel_case_str};
use crate::macro_core::types::delegate_component::{EvalDelegateEntries, EvaluatedDelegateEntry};
use crate::macro_core::types::field::Symbol;
use crate::macro_core::types::keyword::Keyword;
use crate::macro_core::types::keywords::Getter;

/// `type Name = Type` — the obvious provider is `UseType<Type>`, and the
/// component is `{Name}TypeProviderComponent`, the name `#[cgp_type]` derives
/// from the associated type `Name`.
#[derive(Debug, Clone)]
pub struct TypeDelegateShorthand {
    pub type_token: TypeToken,
    pub name: Ident,
    pub eq_token: Eq,
    pub value: Type,
}

impl Parse for TypeDelegateShorthand {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            type_token: input.parse()?,
            name: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl TypeDelegateShorthand {
    /// The component marker. Its tokens use `call_site` so the reference does not
    /// cover the `Name` the user wrote; the entry span still points at `Name`.
    pub fn component_key(&self) -> Type {
        let ident = Ident::new(
            &format!("{}TypeProviderComponent", self.name),
            Span::call_site(),
        );
        syn::parse_quote!(#ident)
    }

    pub fn provider_value(&self) -> syn::Result<Type> {
        let value = &self.value;
        Ok(parse_internal!(#UseType< #value >))
    }
}

impl EvalDelegateEntries for TypeDelegateShorthand {
    fn eval_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        Ok(vec![EvaluatedDelegateEntry {
            table_type: table_type.clone(),
            generics: Default::default(),
            key: self.component_key(),
            value: self.provider_value()?,
            span: self.name.span(),
        }])
    }
}

/// `getter field` — the obvious provider is `UseField` of the field's symbol,
/// and the component is `{Field}GetterComponent`, the name `#[cgp_getter]`
/// derives for a `Has{Field}` trait.
#[derive(Debug, Clone)]
pub struct GetterDelegateShorthand {
    pub getter_token: Keyword<Getter>,
    pub field: Ident,
}

impl Parse for GetterDelegateShorthand {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            getter_token: input.parse()?,
            field: input.parse()?,
        })
    }
}

impl GetterDelegateShorthand {
    pub fn component_key(&self) -> Type {
        let camel = to_camel_case_str(&self.field.unraw().to_string());
        let ident = Ident::new(&format!("{camel}GetterComponent"), Span::call_site());
        syn::parse_quote!(#ident)
    }

    pub fn provider_value(&self) -> syn::Result<Type> {
        let symbol = Symbol::from_ident(self.field.clone());
        Ok(parse_internal!(#UseField< #symbol >))
    }
}

impl EvalDelegateEntries for GetterDelegateShorthand {
    fn eval_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>> {
        Ok(vec![EvaluatedDelegateEntry {
            table_type: table_type.clone(),
            generics: Default::default(),
            key: self.component_key(),
            value: self.provider_value()?,
            span: self.field.span(),
        }])
    }
}
