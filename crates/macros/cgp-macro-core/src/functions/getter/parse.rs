use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Comma, Mut};
use syn::visit_mut::VisitMut;
use syn::{
    Error, FnArg, GenericArgument, Ident, ItemTrait, PathArguments, PathSegment, ReturnType,
    Signature, TraitItem, TraitItemFn, TraitItemType, Type,
};

use crate::functions::{parse_field_type, parse_single_segment_type_path};
use crate::parse_internal;
use crate::types::cgp_getter::{GetterField, ReceiverMode};
use crate::visitors::ReplaceSelfTypeVisitor;

pub fn parse_getter_fields(
    context_type: &Ident,
    consumer_trait: &ItemTrait,
) -> syn::Result<(Vec<GetterField>, Option<TraitItemType>)> {
    let mut fields = Vec::new();
    let mut field_assoc_type: Option<TraitItemType> = None;

    // Extract optional associated type first
    for item in consumer_trait.items.iter() {
        if let TraitItem::Type(item_type) = item {
            if field_assoc_type.is_some() {
                return Err(Error::new(
                    item_type.span(),
                    "at most one associated type is allowed in getter trait",
                ));
            }

            if !item_type.generics.params.is_empty() {
                return Err(Error::new(
                    item_type.generics.params.span(),
                    "associated type in getter trait must not contain generic params",
                ));
            }

            field_assoc_type = Some(item_type.clone());
        }
    }

    for item in consumer_trait.items.iter() {
        match item {
            TraitItem::Fn(method) => {
                let getter_spec = parse_getter_method(
                    context_type,
                    method,
                    &field_assoc_type.as_ref().map(|item| item.ident.clone()),
                )?;

                fields.push(getter_spec);
            }
            TraitItem::Type(_) => {
                // Already processed in the previous loop
            }
            _ => {
                return Err(Error::new(
                    item.span(),
                    "getter trait can only contain getter methods",
                ));
            }
        }
    }

    match (&field_assoc_type, fields.first(), fields.len()) {
        (None, _, _) => {}
        (Some(field_assoc_type), Some(field), 1) => {
            let field_assoc_type_ident = &field_assoc_type.ident;
            let field_type = &field.field_type;

            if field_type != &parse_internal! { Self :: #field_assoc_type_ident }
                && field_type != &parse_internal! { #context_type :: #field_assoc_type_ident }
            {
                return Err(Error::new(
                    field.field_type.span(),
                    "getter method return type must match the associated type",
                ));
            }
        }
        _ => {
            return Err(Error::new(
                consumer_trait.span(),
                "if associated type is defined, exactly one getter method must be defined",
            ));
        }
    }

    Ok((fields, field_assoc_type))
}

fn parse_getter_method(
    context_type: &Ident,
    method: &TraitItemFn,
    field_assoc_type: &Option<Ident>,
) -> syn::Result<GetterField> {
    let signature = &method.sig;

    validate_getter_method_signature(signature)?;

    let field_name = signature.ident.clone();

    let (arg, phantom) = parse_method_args(&signature.inputs)?;

    let (receiver_mode, receiver_mut) = parse_receiver(context_type, arg)?;

    let return_type = parse_return_type(context_type, &signature.output, field_assoc_type)?;

    let (field_type, field_mode) = parse_field_type(&return_type, &receiver_mut)?;

    Ok(GetterField {
        receiver_mode,
        field_name,
        field_type,
        return_type,
        receiver_mut,
        phantom_arg_type: phantom,
        field_mode,
    })
}

fn validate_getter_method_signature(signature: &Signature) -> syn::Result<()> {
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

    Ok(())
}

fn parse_method_args(args: &Punctuated<FnArg, Comma>) -> syn::Result<(&FnArg, Option<Type>)> {
    let args_count = args.len();

    if args_count == 1 {
        let [arg] = parse_fixed_size_args::<1>(args)?;

        Ok((arg, None))
    } else if args_count == 2 {
        let [arg, phantom_arg] = parse_fixed_size_args::<2>(args)?;

        let phantom_arg_type = parse_phantom_arg_type(phantom_arg)?;

        Ok((arg, Some(phantom_arg_type)))
    } else {
        Err(Error::new(
            args.span(),
            "getter method must contain exactly one `&self` argument",
        ))
    }
}

fn parse_fixed_size_args<const I: usize>(
    args: &Punctuated<FnArg, Comma>,
) -> syn::Result<[&FnArg; I]> {
    args.iter()
        .collect::<Vec<&FnArg>>()
        .try_into()
        .map_err(|_| Error::new(args.span(), "expect getter method to contain {I} arguments"))
}

fn parse_phantom_arg_type(phantom_arg: &FnArg) -> syn::Result<Type> {
    match phantom_arg {
        FnArg::Typed(phantom_type) => match phantom_type.ty.as_ref() {
            Type::Path(type_path) => {
                let segment = parse_single_segment_type_path(type_path)?;

                try_parse_phantom_arg_type_path(segment).ok_or_else(|| {
                    Error::new(
                        phantom_type.span(),
                        "only PhantomData is allowed as second argument",
                    )
                })
            }
            _ => Err(Error::new(
                phantom_type.span(),
                "only PhantomData is allowed as second argument",
            )),
        },
        _ => Err(Error::new(
            phantom_arg.span(),
            "optional second argument in getter must be PhantomData",
        )),
    }
}

fn parse_receiver(context_ident: &Ident, arg: &FnArg) -> syn::Result<(ReceiverMode, Option<Mut>)> {
    match arg {
        FnArg::Receiver(receiver) => {
            if receiver.reference.is_none() {
                Err(Error::new(
                    receiver.span(),
                    "first argument to getter method must be a reference to self, i.e. `&self`",
                ))
            } else {
                Ok((ReceiverMode::SelfReceiver, receiver.mutability))
            }
        }
        FnArg::Typed(arg) => match arg.ty.as_ref() {
            Type::Reference(ty) => {
                let mut receiver = ty.elem.clone();

                ReplaceSelfTypeVisitor {
                    replaced_type: &parse_internal!(#context_ident),
                    skip_assoc_types: &Vec::new(),
                }
                .visit_type_mut(&mut receiver);

                Ok((ReceiverMode::Type(receiver), ty.mutability))
            }
            _ => Err(Error::new(
                arg.span(),
                "first argument to getter method must be a reference",
            )),
        },
    }
}

fn parse_return_type(
    context_type: &Ident,
    return_type: &ReturnType,
    field_assoc_type: &Option<Ident>,
) -> syn::Result<Type> {
    match return_type {
        ReturnType::Type(_, ty) => {
            let mut replaced_type = ty.as_ref().clone();

            ReplaceSelfTypeVisitor {
                replaced_type: &parse_internal!(#context_type),
                skip_assoc_types: &Vec::from_iter(field_assoc_type.clone()),
            }
            .visit_type_mut(&mut replaced_type);

            Ok(replaced_type)
        }
        _ => Err(Error::new(
            return_type.span(),
            "return type must be specified",
        )),
    }
}

fn try_parse_phantom_arg_type_path(segment: &PathSegment) -> Option<Type> {
    if segment.ident == "PhantomData"
        && let PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(GenericArgument::Type(ty)) = args.args.first()
    {
        return Some(ty.clone());
    }

    None
}
