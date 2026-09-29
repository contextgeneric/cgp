use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Colon;

use crate::types::check_components::{CheckKey, CheckValue, EvaluatedCheckEntry, TypeWithGenerics};

pub struct CheckEntry {
    pub key: CheckKey,
    pub value: Option<CheckValue>,
}

impl CheckEntry {
    pub fn eval(&self) -> Vec<EvaluatedCheckEntry> {
        let mut entries = Vec::new();

        let keys = self.key.to_keys();

        let component_types_count = keys.len();

        for component_type in keys.iter() {
            if let Some(value) = &self.value {
                let values = value.to_values();

                if values.is_empty() {
                    entries.push(EvaluatedCheckEntry {
                        key: component_type.clone(),
                        value: None,
                        span: component_type.span(),
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
                            key: component_type.clone(),
                            value: Some(TypeWithGenerics {
                                ty: component_param_type.clone(),
                                generics: component_param_generics.clone(),
                            }),
                            span,
                        })
                    }
                }
            } else {
                entries.push(EvaluatedCheckEntry {
                    key: component_type.clone(),
                    value: None,
                    span: component_type.span(),
                })
            }
        }

        entries
    }
}

impl Parse for CheckEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;

        if input.peek(Colon) {
            let _: Colon = input.parse()?;
            let value = input.parse()?;

            Ok(Self {
                key,
                value: Some(value),
            })
        } else {
            Ok(Self { key, value: None })
        }
    }
}
