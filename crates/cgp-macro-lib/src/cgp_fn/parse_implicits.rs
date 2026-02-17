use std::mem;

use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, FnArg, Meta, Pat, PatType};

use crate::cgp_fn::ImplicitArgField;
use crate::derive_getter::parse_field_type;

pub fn extract_implicits_args(
    args: &mut Punctuated<FnArg, Comma>,
) -> syn::Result<Vec<ImplicitArgField>> {
    let mut implicit_args = Vec::new();

    let process_args = mem::take(args);

    for mut arg in process_args.into_iter() {
        if let Some(implicit_arg) = try_parse_implicit_arg(&mut arg)? {
            implicit_args.push(implicit_arg);
        } else {
            args.push(arg);
        }
    }

    Ok(implicit_args)
}

pub fn try_parse_implicit_arg(arg: &mut FnArg) -> syn::Result<Option<ImplicitArgField>> {
    if let FnArg::Typed(arg) = arg {
        let attrs = mem::take(&mut arg.attrs);
        for attr in attrs {
            if is_implicit_attr(&attr) {
                let spec = parse_implicit_arg(arg)?;
                return Ok(Some(spec));
            } else {
                arg.attrs.push(attr);
            }
        }
    }

    Ok(None)
}

pub fn parse_implicit_arg(arg: &PatType) -> syn::Result<ImplicitArgField> {
    let Pat::Ident(pat_ident) = &*arg.pat else {
        return Err(syn::Error::new_spanned(&arg.pat, "Expected an identifier"));
    };

    let field_mut = pat_ident.mutability;

    let (field_type, field_mode) = parse_field_type(&arg.ty, &field_mut)?;

    let spec = ImplicitArgField {
        field_name: pat_ident.ident.clone(),
        field_type,
        field_mut,
        field_mode,
    };

    Ok(spec)
}

pub fn is_implicit_attr(attr: &Attribute) -> bool {
    match &attr.meta {
        Meta::Path(path) => path.is_ident("implicit"),
        _ => false,
    }
}
