use alloc::vec::Vec;

use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{
    parse_quote, Error, FnArg, GenericArgument, Ident, ItemTrait, PathArguments, ReturnType,
    TraitItem, Type, TypePath,
};

use crate::derive_component::replace_self_type;
use crate::getter_component::getter_field::GetterField;
use crate::getter_component::FieldMode;

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

                let args_count = signature.inputs.len();

                let (arg, phantom) = if args_count == 1 {
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

                    (arg, None)
                } else if args_count == 2 {
                    let [arg, phantom_arg]: [&FnArg; 2] = signature
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

                    match phantom_arg {
                        FnArg::Typed(phantom_type) => (arg, Some(phantom_type.ty.as_ref().clone())),
                        _ => {
                            return Err(Error::new(
                                signature.inputs.span(),
                                "optional second argument in getter must be PhantomData",
                            ));
                        }
                    }
                } else {
                    return Err(Error::new(
                        signature.inputs.span(),
                        "getter method must contain exactly one `&self` argument",
                    ));
                };

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

                let (field_type, field_mode) = match &signature.output {
                    ReturnType::Type(_, ty) => {
                        let return_type = ty.as_ref().clone();
                        match &return_type {
                            Type::Reference(type_ref) => {
                                if type_ref.mutability.is_some() != field_mut.is_some() {
                                    return Err(Error::new(
                                        type_ref.span(),
                                        "return type have the same mutability as the self reference",
                                    ));
                                }

                                if type_ref == &parse_quote! { &str } {
                                    // Special case to handle &str as String field

                                    let field_type: Type = parse_quote! { String };

                                    (field_type, FieldMode::Str)
                                } else {
                                    let field_type: Type = type_ref.elem.as_ref().clone();

                                    (field_type, FieldMode::Reference)
                                }
                            }
                            Type::Path(type_path) => {
                                if try_parse_option_ref(type_path).is_some() {
                                    (return_type, FieldMode::AsRef)
                                } else {
                                    (return_type, FieldMode::Clone)
                                }
                            }
                            _ => {
                                return Err(Error::new(
                                    return_type.span(),
                                    "return type must be a reference",
                                ))
                            }
                        }
                    }
                    _ => {
                        return Err(Error::new(
                            signature.span(),
                            "return type must be specified",
                        ))
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
                    phantom_arg_type: phantom,
                    field_mode,
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

pub fn try_parse_option_ref(type_path: &TypePath) -> Option<&Type> {
    let m_segment = type_path.path.segments.iter().next();

    if let Some(segment) = m_segment {
        if segment.ident == "Option" {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                if let Some(GenericArgument::Type(Type::Reference(type_ref))) = args.args.first() {
                    return Some(type_ref.elem.as_ref());
                }
            }
        }
    }

    None
}
