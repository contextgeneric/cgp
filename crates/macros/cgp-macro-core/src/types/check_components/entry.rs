use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Colon;

use crate::types::check_components::{CheckKey, CheckValue, EvaluatedCheckEntry};
use crate::types::generics::ImplGenerics;

pub struct CheckEntry {
    pub key: CheckKey,
    pub colon: Colon,
    pub value: CheckValue,
}

impl CheckEntry {
    pub fn eval(&self) -> Vec<EvaluatedCheckEntry> {
        let mut entries = Vec::new();

        let keys = self.key.to_keys();
        let values = self.value.to_values();

        let component_types_count = keys.len();

        for component_type in keys.iter() {
            if values.is_empty() {
                entries.push(EvaluatedCheckEntry {
                    component_type: component_type.clone(),
                    component_params: None,
                    span: component_type.span(),
                    generics: ImplGenerics::default(),
                })
            } else {
                let component_params_count = values.len();

                for component_param in values.iter() {
                    let component_param_type = &component_param.ty;
                    let component_param_generics = &component_param.generics;

                    let span = if component_types_count >= component_params_count {
                        component_type.span()
                    } else {
                        component_param_type.span()
                    };

                    entries.push(EvaluatedCheckEntry {
                        component_type: component_type.clone(),
                        component_params: Some(component_param_type.clone()),
                        span,
                        generics: component_param_generics.clone(),
                    })
                }
            }
        }

        todo!()
    }
}

impl Parse for CheckEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        let colon = input.parse()?;
        let value = input.parse()?;

        Ok(Self { key, colon, value })
    }
}
