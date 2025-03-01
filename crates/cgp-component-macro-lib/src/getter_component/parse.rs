use alloc::vec::Vec;

use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{parse_quote, Error, FnArg, Ident, ItemTrait, ReturnType, TraitItem, Type};

use crate::derive_component::replace_self_type;
use crate::getter_component::getter_field::GetterField;

pub fn parse_getter_fields(
    context_type: &Ident,
    consumer_trait: &ItemTrait,
) -> syn::Result<Vec<GetterField>> {
    let mut fields = Vec::new();

    for item in consumer_trait.items.iter() {
        match item {
            TraitItem::Fn(method) => {
                let signature = &method.sig;

                if signature.constness.is_some() {
                    return Err(Error::new(
                        signature.constness.span(),
                        "getter method must not be const fn",
                    ));
                }

                if signature.asyncness.is_some() {
                    return Err(Error::new(
                        signature.asyncness.span(),
                        "getter method must not be async fn",
                    ));
                }

                if signature.unsafety.is_some() {
                    return Err(Error::new(
                        signature.unsafety.span(),
                        "getter method must not be unsafe fn",
                    ));
                }

                if !signature.generics.params.is_empty() {
                    return Err(Error::new(
                        signature.generics.params.span(),
                        "getter method must not contain generic param",
                    ));
                }

                if signature.generics.where_clause.is_some() {
                    return Err(Error::new(
                        signature.generics.where_clause.span(),
                        "getter method must not contain where clause",
                    ));
                }

                let field_name = signature.ident.clone();

                let [arg]: [&FnArg; 1] = signature
                    .inputs
                    .iter()
                    .collect::<Vec<&FnArg>>()
                    .try_into()
                    .map_err(|_| {
                        Error::new(
                            signature.inputs.span(),
                            "getter method must contain exactly one `&self` argument",
                        )
                    })?;

                let field_mut = match arg {
                    FnArg::Receiver(receiver) => {
                        if receiver.reference.is_none() {
                            return Err(Error::new(
                                receiver.span(),
                                "first argument to getter method must be a reference to self, i.e. `&self`"
                            ));
                        }

                        receiver.mutability
                    }
                    _ => {
                        return Err(Error::new(
                            arg.span(),
                            "first argument to getter method must be `&self`",
                        ))
                    }
                };

                let field_type: Type = match &signature.output {
                    ReturnType::Default => parse_quote!(()),
                    ReturnType::Type(_, ty) => {
                        let ty = ty.as_ref().clone();
                        match &ty {
                            Type::Reference(type_ref) => {
                                if type_ref.mutability.is_some() != field_mut.is_some() {
                                    return Err(Error::new(
                                        type_ref.span(),
                                        "return type have the same mutability as the self reference",
                                    ));
                                }

                                type_ref.elem.as_ref().clone()
                            }
                            _ => {
                                return Err(Error::new(
                                    ty.span(),
                                    "return type must be a reference",
                                ))
                            }
                        }
                    }
                };

                let provider_type: Type = syn::parse2(replace_self_type(
                    field_type.to_token_stream(),
                    context_type,
                    &Vec::new(),
                ))?;

                fields.push(GetterField {
                    field_name,
                    provider_type,
                    field_mut,
                })
            }
            _ => {
                return Err(Error::new(
                    item.span(),
                    "getter trait can only contain getter methods",
                ))
            }
        }
    }

    Ok(fields)
}
