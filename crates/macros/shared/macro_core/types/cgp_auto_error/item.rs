use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::visit_mut::{VisitMut, visit_type_mut};
use syn::{
    Error, GenericParam, Generics, Ident, ImplItem, ImplItemFn, Item, ItemImpl, Pat, Type,
    TypePath, parse_quote,
};

use crate::macro_core::exports::{
    ErrorRaiser, ErrorRaiserComponent, ErrorTypeProvider, ErrorTypeProviderComponent, ErrorWrapper,
    ErrorWrapperComponent, HasErrorType,
};
use crate::macro_core::functions::{override_item_span, parse_internal};
use crate::macro_core::types::cgp_provider::{ItemCgpProvider, ProviderArgs};
use crate::macro_core::types::empty_struct::EmptyStruct;

/// `#[cgp_auto_error]` pipeline: an inherent impl whose `Error` type and
/// `raise_error` / `wrap_error` bodies become the three error providers.
pub struct ItemCgpAutoError {
    pub provider: Type,
    pub provider_ident: Ident,
    pub error_ty: Type,
    pub raise_error: ImplItemFn,
    pub wrap_error: ImplItemFn,
}

impl ItemCgpAutoError {
    pub fn parse(item_impl: &ItemImpl) -> syn::Result<Self> {
        if item_impl.trait_.is_some() {
            return Err(Error::new_spanned(
                &item_impl.self_ty,
                "apply #[cgp_auto_error] to an inherent impl that names the provider, not a trait impl",
            ));
        }

        if item_impl.unsafety.is_some() || item_impl.defaultness.is_some() {
            return Err(Error::new_spanned(
                &item_impl.self_ty,
                "#[cgp_auto_error] does not support unsafe or default impls",
            ));
        }

        if !item_impl.generics.params.is_empty() || item_impl.generics.where_clause.is_some() {
            return Err(Error::new_spanned(
                &item_impl.generics,
                "put generic parameters on raise_error and wrap_error, not on the impl",
            ));
        }

        reject_non_doc_attrs(&item_impl.attrs)?;

        let provider_ident = plain_ident(&item_impl.self_ty)?;
        let mut error_ty = None;
        let mut raise_error = None;
        let mut wrap_error = None;

        for item in &item_impl.items {
            match item {
                ImplItem::Type(assoc) => {
                    if assoc.ident != "Error" {
                        return Err(Error::new_spanned(
                            &assoc.ident,
                            "the only associated type in #[cgp_auto_error] is `Error`",
                        ));
                    }
                    if error_ty.is_some() {
                        return Err(Error::new_spanned(
                            &assoc.ident,
                            "`Error` is already defined",
                        ));
                    }
                    if !assoc.generics.params.is_empty() {
                        return Err(Error::new_spanned(
                            &assoc.generics,
                            "`Error` cannot take generic parameters",
                        ));
                    }
                    error_ty = Some(assoc.ty.clone());
                }
                ImplItem::Fn(func) => match func.sig.ident.to_string().as_str() {
                    "raise_error" => {
                        if raise_error.is_some() {
                            return Err(Error::new_spanned(
                                &func.sig.ident,
                                "`raise_error` is already defined",
                            ));
                        }
                        raise_error = Some(func.clone());
                    }
                    "wrap_error" => {
                        if wrap_error.is_some() {
                            return Err(Error::new_spanned(
                                &func.sig.ident,
                                "`wrap_error` is already defined",
                            ));
                        }
                        wrap_error = Some(func.clone());
                    }
                    _ => {
                        return Err(Error::new_spanned(
                            &func.sig.ident,
                            "#[cgp_auto_error] accepts only `raise_error` and `wrap_error`",
                        ));
                    }
                },
                _ => {
                    return Err(Error::new_spanned(
                        item,
                        "#[cgp_auto_error] accepts an `Error` type, `raise_error`, and `wrap_error`",
                    ));
                }
            }
        }

        let error_ty = error_ty
            .ok_or_else(|| Error::new_spanned(&item_impl.self_ty, "define `type Error = ...`"))?;
        let raise_error = raise_error.ok_or_else(|| {
            Error::new_spanned(
                &item_impl.self_ty,
                "define `raise_error`, the body of `CanRaiseError`",
            )
        })?;
        let wrap_error = wrap_error.ok_or_else(|| {
            Error::new_spanned(
                &item_impl.self_ty,
                "define `wrap_error`, the body of `CanWrapError`",
            )
        })?;

        validate_error_fn(&raise_error, 1)?;
        validate_error_fn(&wrap_error, 2)?;

        if !is_bare_error(&raise_error.sig.output) {
            return Err(Error::new_spanned(
                &raise_error.sig.output,
                "raise_error must return `Error`",
            ));
        }

        let wrap_args = typed_args(&wrap_error)?;
        if !is_bare_error_type(&wrap_args[0].1) {
            return Err(Error::new_spanned(
                &wrap_args[0].1,
                "wrap_error's first argument must be `Error`",
            ));
        }
        if !is_bare_error(&wrap_error.sig.output) {
            return Err(Error::new_spanned(
                &wrap_error.sig.output,
                "wrap_error must return `Error`",
            ));
        }

        Ok(Self {
            provider: (*item_impl.self_ty).clone(),
            provider_ident,
            error_ty,
            raise_error,
            wrap_error,
        })
    }

    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let context = Ident::new("__Context__", Span::call_site());

        let provider_struct = EmptyStruct {
            ident: self.provider_ident.clone(),
            generics: Generics::default(),
        };

        let mut items = vec![provider_struct.to_item_struct().into()];
        items.extend(self.type_provider_items(&context)?);
        items.extend(self.raise_provider_items(&context)?);
        items.extend(self.wrap_provider_items(&context)?);
        Ok(items)
    }

    fn type_provider_items(&self, context: &Ident) -> syn::Result<Vec<Item>> {
        let provider = &self.provider;
        let error_ty = &self.error_ty;
        let error_type_provider = ErrorTypeProvider;

        let item_impl: ItemImpl = parse_internal! {
            impl <#context> #error_type_provider <#context> for #provider {
                type Error = #error_ty;
            }
        };

        lower_provider(ErrorTypeProviderComponent, item_impl, self.error_ty.span())
    }

    fn raise_provider_items(&self, context: &Ident) -> syn::Result<Vec<Item>> {
        let mut method = self.raise_error.clone();
        rewrite_abstract_error(&mut method, context);
        let args = typed_args(&method)?;
        let (pat, source_ty) = &args[0];
        let generics = lifted_generics(&method, context, &self.error_ty)?;
        let (impl_generics, _, where_clause) = generics.split_for_impl();
        let provider = &self.provider;
        let error_raiser = ErrorRaiser;
        let body = &method.block;

        let item_impl: ItemImpl = parse_internal! {
            impl #impl_generics #error_raiser <#context, #source_ty> for #provider
            #where_clause
            {
                #[track_caller]
                fn raise_error(#pat: #source_ty) -> #context::Error
                #body
            }
        };

        lower_provider(ErrorRaiserComponent, item_impl, method.sig.ident.span())
    }

    fn wrap_provider_items(&self, context: &Ident) -> syn::Result<Vec<Item>> {
        let mut method = self.wrap_error.clone();
        rewrite_abstract_error(&mut method, context);
        let args = typed_args(&method)?;
        let (error_pat, error_ty) = &args[0];
        let (detail_pat, detail_ty) = &args[1];
        let generics = lifted_generics(&method, context, &self.error_ty)?;
        let (impl_generics, _, where_clause) = generics.split_for_impl();
        let provider = &self.provider;
        let error_wrapper = ErrorWrapper;
        let body = &method.block;

        let item_impl: ItemImpl = parse_internal! {
            impl #impl_generics #error_wrapper <#context, #detail_ty> for #provider
            #where_clause
            {
                #[track_caller]
                fn wrap_error(#error_pat: #error_ty, #detail_pat: #detail_ty) -> #context::Error
                #body
            }
        };

        lower_provider(ErrorWrapperComponent, item_impl, method.sig.ident.span())
    }
}

fn lower_provider(
    component: impl quote::ToTokens,
    mut item_impl: ItemImpl,
    span: Span,
) -> syn::Result<Vec<Item>> {
    if let Some((_, _, for_token)) = &mut item_impl.trait_ {
        for_token.span = span;
    }

    let item_impl = override_item_span(span, &item_impl)?;
    let component_type = parse_internal!(#component);

    let lowered = ItemCgpProvider {
        args: ProviderArgs {
            new: None,
            component_type: Some(component_type),
        },
        item_impl,
    }
    .lower()?;

    Ok(vec![
        lowered.item_impl.into(),
        lowered.is_provider_for_impl.into(),
    ])
}

fn lifted_generics(method: &ImplItemFn, context: &Ident, error_ty: &Type) -> syn::Result<Generics> {
    for param in &method.sig.generics.params {
        match param {
            GenericParam::Const(param) => {
                return Err(Error::new_spanned(
                    param,
                    "const generics are not supported on #[cgp_auto_error] methods",
                ));
            }
            GenericParam::Type(param) if param.ident == "Error" => {
                return Err(Error::new_spanned(
                    &param.ident,
                    "`Error` names the abstract error type in this definition",
                ));
            }
            _ => {}
        }
    }

    let mut generics = method.sig.generics.clone();
    generics.params.insert(0, parse_internal!(#context));

    let has_error_type = HasErrorType;
    let bound = parse_internal! {
        #context: #has_error_type < Error = #error_ty >
    };
    generics.make_where_clause().predicates.insert(0, bound);

    Ok(generics)
}

fn validate_error_fn(method: &ImplItemFn, expected_args: usize) -> syn::Result<()> {
    if method.sig.receiver().is_some() {
        return Err(Error::new_spanned(
            &method.sig,
            "raise_error and wrap_error take no receiver; they are provider functions",
        ));
    }
    if method.sig.asyncness.is_some() || method.sig.unsafety.is_some() {
        return Err(Error::new_spanned(
            &method.sig,
            "raise_error and wrap_error must be synchronous safe functions",
        ));
    }
    let argc = method.sig.inputs.len();
    if argc != expected_args {
        return Err(Error::new_spanned(
            &method.sig,
            format!("expected {expected_args} argument(s), found {argc}"),
        ));
    }
    Ok(())
}

fn typed_args(method: &ImplItemFn) -> syn::Result<Vec<(Pat, Type)>> {
    let mut args = Vec::new();
    for input in &method.sig.inputs {
        let syn::FnArg::Typed(pat_type) = input else {
            return Err(Error::new_spanned(
                input,
                "raise_error and wrap_error take no receiver",
            ));
        };
        args.push(((*pat_type.pat).clone(), (*pat_type.ty).clone()));
    }
    Ok(args)
}

fn is_bare_error(output: &syn::ReturnType) -> bool {
    match output {
        syn::ReturnType::Type(_, ty) => is_bare_error_type(ty),
        syn::ReturnType::Default => false,
    }
}

fn is_bare_error_type(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath { qself: None, path }) => {
            path.leading_colon.is_none()
                && path.segments.len() == 1
                && path.segments[0].ident == "Error"
                && path.segments[0].arguments.is_empty()
        }
        _ => false,
    }
}

fn rewrite_abstract_error(method: &mut ImplItemFn, context: &Ident) {
    let mut replace = ReplaceBareError {
        context: context.clone(),
    };
    replace.visit_impl_item_fn_mut(method);
}

struct ReplaceBareError {
    context: Ident,
}

impl VisitMut for ReplaceBareError {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        if is_bare_error_type(ty) {
            let context = &self.context;
            *ty = parse_quote!(#context::Error);
            return;
        }
        visit_type_mut(self, ty);
    }
}

fn plain_ident(ty: &Type) -> syn::Result<Ident> {
    let Type::Path(type_path) = ty else {
        return Err(Error::new_spanned(
            ty,
            "the provider name must be a single identifier",
        ));
    };
    if type_path.qself.is_some()
        || type_path.path.leading_colon.is_some()
        || type_path.path.segments.len() != 1
    {
        return Err(Error::new_spanned(
            ty,
            "the provider name must be a single identifier",
        ));
    }
    let segment = &type_path.path.segments[0];
    if !segment.arguments.is_empty() {
        return Err(Error::new_spanned(
            segment,
            "the provider name cannot take generic arguments",
        ));
    }
    Ok(segment.ident.clone())
}

fn reject_non_doc_attrs(attrs: &[syn::Attribute]) -> syn::Result<()> {
    for attr in attrs {
        if !attr.path().is_ident("doc") {
            return Err(Error::new_spanned(
                attr,
                "#[cgp_auto_error] does not accept helper attributes",
            ));
        }
    }
    Ok(())
}
