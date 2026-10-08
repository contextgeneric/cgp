use std::mem;

use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::visit::{self, Visit};
use syn::{Attribute, FnArg, Meta, Pat, PatIdent, PatType, Receiver};

use crate::macro_core::functions::parse_field_type;
use crate::macro_core::types::implicits::{ImplicitArgField, ImplicitArgFields};

pub fn extract_and_parse_implicit_args(
    args: &mut Punctuated<FnArg, Comma>,
) -> syn::Result<ImplicitArgFields> {
    let implicit_fn_args = extract_implicit_args(args)?;

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
    // `parse_field_type` derives the field-access mutability from the reference in
    // the argument type — the outer `&mut` of a `&mut T`/`&mut [T]`, or the inner
    // `&mut` of an `Option<&mut T>` — and rejects a mutable read without a `&mut
    // self` receiver. The receiver's own mutability never forces a mutable read of
    // an immutably-typed argument.
    parse_named_field_arg(arg, &receiver.mutability)
}

pub fn extract_implicit_args(args: &mut Punctuated<FnArg, Comma>) -> syn::Result<Vec<PatType>> {
    let mut implicit_args = Vec::new();

    let process_args = mem::take(args);

    for arg in process_args.into_iter() {
        if let FnArg::Typed(mut arg) = arg {
            if is_implicit_arg(&mut arg)? {
                implicit_args.push(arg);
            } else {
                args.push(FnArg::Typed(arg));
            }
        } else {
            args.push(arg);
        }
    }

    Ok(implicit_args)
}

pub fn is_implicit_arg(arg: &mut PatType) -> syn::Result<bool> {
    let mut res = false;

    let attrs = mem::take(&mut arg.attrs);

    for attr in attrs {
        if is_implicit_attr(&attr) {
            res = true;
        } else if let Some(name) = context_arg_name(&attr) {
            // `#[implicit]` and `#[field]` are bare markers for the same read.
            // A list or name-value form is a mistake. Reject it here rather than
            // leaving the stray attribute on the parameter, where it would surface
            // far downstream as an obscure "cannot find attribute" error.
            return Err(syn::Error::new_spanned(
                &attr,
                format!("`#[{name}]` does not take any arguments; write it as a bare `#[{name}]`"),
            ));
        } else {
            arg.attrs.push(attr);
        }
    }

    Ok(res)
}

/// `#[implicit]` and `#[field]` name the same context-field read. The bare path
/// form is the marker; any other form of either name is rejected by
/// [`is_implicit_arg`].
pub fn is_implicit_attr(attr: &Attribute) -> bool {
    match &attr.meta {
        Meta::Path(path) => is_context_arg_ident(path.get_ident()),
        _ => false,
    }
}

fn context_arg_name(attr: &Attribute) -> Option<syn::Ident> {
    let ident = attr.path().get_ident()?;
    if is_context_arg_ident(Some(ident)) {
        Some(ident.clone())
    } else {
        None
    }
}

fn is_context_arg_ident(ident: Option<&syn::Ident>) -> bool {
    match ident {
        Some(ident) => ident == "implicit" || ident == "field",
        None => false,
    }
}

/// Parse a context-field argument for a provider that has no `self` receiver,
/// such as a `#[cgp_computer]` function. The value is read from a shared
/// `&Context`, so a mutable reference is rejected.
pub fn parse_context_arg(arg: &PatType) -> syn::Result<ImplicitArgField> {
    parse_named_field_arg(arg, &None).map_err(|err| {
        if err.to_string().contains("&mut self is required") {
            syn::Error::new(
                err.span(),
                "a context-field argument is read from a shared `&Context`, so it cannot be a mutable reference",
            )
        } else {
            err
        }
    })
}

fn parse_named_field_arg(
    arg: &PatType,
    receiver_mut: &Option<syn::token::Mut>,
) -> syn::Result<ImplicitArgField> {
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
    let (field_type, field_mode, field_mut) = parse_field_type(&arg_type, receiver_mut)?;

    Ok(ImplicitArgField {
        field_name: pat_ident.ident.clone(),
        field_type,
        field_mut,
        field_mode,
        arg_type,
    })
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
