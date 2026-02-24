use quote::quote;
use syn::parse::Parse;
use syn::visit_mut::{VisitMut, visit_item_impl_mut};
use syn::{Expr, ExprMethodCall, ItemImpl, Type, parse2};

pub fn transform_provider_call(item_impl: &mut ItemImpl) -> syn::Result<()> {
    let mut visitor = TransformProviderCallVisitor::default();
    visit_item_impl_mut(&mut visitor, item_impl);

    if let Some(err) = visitor.error {
        Err(err)
    } else {
        Ok(())
    }
}

#[derive(Default)]
pub struct TransformProviderCallVisitor {
    pub error: Option<syn::Error>,
}

impl VisitMut for TransformProviderCallVisitor {
    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        if self.error.is_some() {
            return;
        }

        if let syn::Expr::MethodCall(method_call) = expr {
            match visit_provider_call(method_call) {
                Ok(Some(new_expr)) => {
                    *expr = new_expr;
                }
                Ok(None) => {}
                Err(e) => {
                    self.error = Some(e);
                }
            }
        }

        syn::visit_mut::visit_expr_mut(self, expr);
    }
}

fn visit_provider_call(expr: &ExprMethodCall) -> syn::Result<Option<Expr>> {
    let attributes = expr.attrs.clone();
    let mut out_attributes = Vec::new();

    let mut m_use_provider = None;

    for attribute in attributes {
        if attribute.path().is_ident("use_provider") {
            if m_use_provider.is_some() {
                return Err(syn::Error::new_spanned(
                    attribute,
                    "Multiple #[use_provider] attributes found",
                ));
            }

            m_use_provider = Some(attribute.parse_args_with(Type::parse)?);
        } else {
            out_attributes.push(attribute);
        }
    }

    if let Some(provider_type) = m_use_provider {
        let mut args = expr.args.clone();
        args.insert(0, expr.receiver.as_ref().clone());

        let method = &expr.method;
        let turbofish = &expr.turbofish;

        let new_expr: Expr = parse2(quote! {
            #provider_type::#method #turbofish ( #args )
        })?;

        Ok(Some(new_expr))
    } else {
        Ok(None)
    }
}
