use alloc::vec::Vec;

use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Comma, Mut};
use syn::{
    parse2, parse_quote, Error, FnArg, GenericArgument, Ident, ItemTrait, PathArguments,
    PathSegment, ReturnType, Signature, TraitItem, TraitItemFn, Type, TypePath,
};

use crate::derive_component::replace_self_type;
use crate::derive_getter::getter_field::GetterField;
use crate::derive_getter::FieldMode;

pub fn parse_getter_fields(
    context_type: &Ident,
    consumer_trait: &ItemTrait,
) -> syn::Result<Vec<GetterField>> {
    let mut fields = Vec::new();

    for item in consumer_trait.items.iter() {
        match item {
            TraitItem::Fn(method) => {
                let getter_spec = parse_getter_method(context_type, method)?;

                fields.push(getter_spec);
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

fn parse_getter_method(context_type: &Ident, method: &TraitItemFn) -> syn::Result<GetterField> {
    let signature = &method.sig;

    validate_getter_method_signature(signature)?;

    let field_name = signature.ident.clone();

    let (arg, phantom) = parse_method_args(&signature.inputs)?;

    let field_mut = parse_field_mut(arg)?;

    let return_type = parse_return_type(context_type, &signature.output)?;

    let (field_type, field_mode) = parse_field_type(&return_type, &field_mut)?;

    Ok(GetterField {
        field_name,
        field_type,
        return_type,
        field_mut,
        phantom_arg_type: phantom,
        field_mode,
    })
}

pub fn validate_getter_method_signature(signature: &Signature) -> syn::Result<()> {
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
        return Err(Error::new(
            args.span(),
            "getter method must contain exactly one `&self` argument",
        ));
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

fn parse_field_mut(arg: &FnArg) -> syn::Result<Option<Mut>> {
    match arg {
        FnArg::Receiver(receiver) => {
            if receiver.reference.is_none() {
                Err(Error::new(
                    receiver.span(),
                    "first argument to getter method must be a reference to self, i.e. `&self`",
                ))
            } else {
                Ok(receiver.mutability)
            }
        }
        _ => Err(Error::new(
            arg.span(),
            "first argument to getter method must be `&self`",
        )),
    }
}

fn parse_return_type(context_type: &Ident, return_type: &ReturnType) -> syn::Result<Type> {
    match return_type {
        ReturnType::Type(_, ty) => parse2(replace_self_type(
            ty.to_token_stream(),
            context_type,
            &Vec::new(),
        )),
        _ => Err(Error::new(
            return_type.span(),
            "return type must be specified",
        )),
    }
}

fn parse_field_type(return_type: &Type, field_mut: &Option<Mut>) -> syn::Result<(Type, FieldMode)> {
    match &return_type {
        Type::Reference(type_ref) => {
            if type_ref.mutability.is_some() != field_mut.is_some() {
                return Err(Error::new(
                    type_ref.span(),
                    "return type have the same mutability as the self reference",
                ));
            }

            if type_ref.elem.as_ref() == &parse_quote! { str } {
                // Special case to handle &str as String field

                let field_type: Type = parse_quote! { String };

                Ok((field_type, FieldMode::Str))
            } else {
                let field_type: Type = type_ref.elem.as_ref().clone();

                Ok((field_type, FieldMode::Reference))
            }
        }
        Type::Path(type_path) => {
            if let Some(field_type) = try_parse_option_ref(type_path) {
                Ok((
                    parse2(quote! { Option< #field_type > })?,
                    FieldMode::OptionRef,
                ))
            } else {
                Ok((return_type.clone(), FieldMode::Clone))
            }
        }
        _ => Err(Error::new(
            return_type.span(),
            "return type must be a reference",
        )),
    }
}

fn parse_single_segment_type_path(type_path: &TypePath) -> syn::Result<&PathSegment> {
    let [segment]: [&PathSegment; 1] = type_path
        .path
        .segments
        .iter()
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| {
            Error::new(
                type_path.span(),
                "type path must contain exactly one path segment",
            )
        })?;

    Ok(segment)
}

fn try_parse_phantom_arg_type_path(segment: &PathSegment) -> Option<Type> {
    if segment.ident == "PhantomData" {
        if let PathArguments::AngleBracketed(args) = &segment.arguments {
            if let Some(GenericArgument::Type(ty)) = args.args.first() {
                return Some(ty.clone());
            }
        }
    }

    None
}

fn try_parse_option_ref(type_path: &TypePath) -> Option<&Type> {
    let segment = parse_single_segment_type_path(type_path).ok()?;

    if segment.ident == "Option" {
        if let PathArguments::AngleBracketed(args) = &segment.arguments {
            if let Some(GenericArgument::Type(Type::Reference(type_ref))) = args.args.first() {
                return Some(type_ref.elem.as_ref());
            }
        }
    }

    None
}
