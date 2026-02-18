use std::mem;

use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit::{self, Visit};
use syn::{Attribute, FnArg, Meta, Pat, PatIdent, PatType, Receiver};

use crate::cgp_fn::ImplicitArgField;
use crate::derive_getter::parse_field_type;

pub fn extract_implicits_args(
    receiver: &Receiver,
    args: &mut Punctuated<FnArg, Comma>,
) -> syn::Result<Vec<ImplicitArgField>> {
    let implicit_fn_args = extract_implicit_args(args);

    if receiver.mutability.is_some() && implicit_fn_args.len() > 1 {
        return Err(syn::Error::new_spanned(
            &args,
            "Only one mutable implicit argument is allowed when self is mutable",
        ));
    }

    let mut implicit_args = Vec::new();

    for arg in implicit_fn_args {
        let spec = parse_implicit_arg(receiver, &arg)?;
        implicit_args.push(spec);
    }

    Ok(implicit_args)
}

pub fn parse_implicit_arg(receiver: &Receiver, arg: &PatType) -> syn::Result<ImplicitArgField> {
    let Pat::Ident(pat_ident) = &*arg.pat else {
        return Err(syn::Error::new_spanned(&arg.pat, "Expected an identifier"));
    };

    if has_mut_pattern(&arg.pat) {
        return Err(syn::Error::new_spanned(
            &arg.pat,
            "Mutable variables are not allowed in implicit arguments. (Explicitly clone a `&` reference if you want a mutable local copy of the value)",
        ));
    }

    let arg_type = arg.ty.as_ref().clone();

    let field_mut = receiver.mutability;

    let (field_type, field_mode) = parse_field_type(&arg_type, &field_mut)?;

    let spec = ImplicitArgField {
        field_name: pat_ident.ident.clone(),
        field_type,
        field_mut,
        field_mode,
        arg_type,
    };

    Ok(spec)
}

pub fn extract_implicit_args(args: &mut Punctuated<FnArg, Comma>) -> Vec<PatType> {
    let mut implicit_args = Vec::new();

    let process_args = mem::take(args);

    for arg in process_args.into_iter() {
        if let FnArg::Typed(mut arg) = arg {
            if is_implicit_arg(&mut arg) {
                implicit_args.push(arg);
            } else {
                args.push(FnArg::Typed(arg));
            }
        } else {
            args.push(arg);
        }
    }

    implicit_args
}

pub fn is_implicit_arg(arg: &mut PatType) -> bool {
    let mut res = false;

    let attrs = mem::take(&mut arg.attrs);

    for attr in attrs {
        if is_implicit_attr(&attr) {
            res = true;
        } else {
            arg.attrs.push(attr);
        }
    }

    res
}

pub fn is_implicit_attr(attr: &Attribute) -> bool {
    match &attr.meta {
        Meta::Path(path) => path.is_ident("implicit"),
        _ => false,
    }
}

pub fn has_mut_pattern(pat: &Pat) -> bool {
    let mut checker = MutChecker { has_mut: false };
    checker.visit_pat(pat);
    checker.has_mut
}

struct MutChecker {
    has_mut: bool,
}

impl<'ast> Visit<'ast> for MutChecker {
    fn visit_pat_ident(&mut self, node: &'ast PatIdent) {
        if node.mutability.is_some() {
            self.has_mut = true;
        }
        // Continue walking through the rest of the pattern
        visit::visit_pat_ident(self, node);
    }
}
