use std::mem;

use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit::{self, Visit};
use syn::{Attribute, FnArg, Meta, Pat, PatIdent, PatType, Receiver, Type};

use crate::functions::parse_field_type;
use crate::types::implicits::{ImplicitArgField, ImplicitArgFields};

pub fn extract_and_parse_implicit_args(
    args: &mut Punctuated<FnArg, Comma>,
) -> syn::Result<ImplicitArgFields> {
    let implicit_fn_args = extract_implicit_args(args);

    if implicit_fn_args.is_empty() {
        return Ok(ImplicitArgFields::default());
    }

    let Some(FnArg::Receiver(receiver)) = args.first() else {
        return Err(syn::Error::new_spanned(
            &args,
            "The first argument of a function with implicit arguments must be `self`",
        ));
    };

    let mut implicit_args = Vec::new();

    for arg in implicit_fn_args {
        let spec = parse_implicit_arg(receiver, &arg)?;
        implicit_args.push(spec);
    }

    // A `&mut` implicit reads through `get_field_mut`, which borrows the whole
    // context exclusively for the rest of the body, so it cannot coexist with any
    // other implicit read — the emitted impl would borrow the context mutably and
    // (im)mutably at once and fail to compile. Purely immutable implicits are all
    // shared borrows and combine freely, so the constraint applies only once a
    // mutable one is present.
    let has_mutable = implicit_args.iter().any(|field| field.field_mut.is_some());

    if has_mutable && implicit_args.len() > 1 {
        return Err(syn::Error::new_spanned(
            &args,
            "a `&mut` implicit argument must be the only implicit argument, since its mutable borrow of the context conflicts with reading any other field",
        ));
    }

    Ok(ImplicitArgFields::new(implicit_args))
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

    let (field_type, field_mode) = parse_field_type(&arg_type, &receiver.mutability)?;

    // The field is read mutably only when the argument itself is a `&mut`
    // reference. The receiver's mutability gates *whether* a `&mut` argument is
    // allowed (checked in `parse_field_type`), but a `&mut self` receiver does not
    // by itself force a mutable read of an immutably-typed argument.
    let field_mut = match &arg_type {
        Type::Reference(type_ref) => type_ref.mutability,
        _ => None,
    };

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
