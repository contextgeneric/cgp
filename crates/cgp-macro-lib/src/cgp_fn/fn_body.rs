use quote::quote;
use syn::{Block, parse2};

use crate::cgp_fn::ImplicitArgField;
use crate::derive_getter::extend_call_expr;
use crate::symbol::symbol_from_string;

pub fn inject_implicit_args(args: &[ImplicitArgField], body: &mut Block) -> syn::Result<()> {
    for arg in args.iter().rev() {
        inject_implicit_arg(arg, body)?;
    }
    Ok(())
}

pub fn inject_implicit_arg(arg: &ImplicitArgField, body: &mut Block) -> syn::Result<()> {
    let field_name = &arg.field_name;

    let field_symbol = symbol_from_string(&field_name.to_string());

    let call_expr = if arg.field_mut.is_none() {
        quote! {
            self.get_field(::core::marker::PhantomData::< #field_symbol >)
        }
    } else {
        quote! {
            self.get_field_mut(::core::marker::PhantomData::< #field_symbol >)
        }
    };

    let call_expr = extend_call_expr(call_expr, &arg.field_mode, &arg.field_mut);

    let statement = parse2(quote! {
        let #field_name = #call_expr;
    })?;

    body.stmts.insert(0, statement);

    Ok(())
}
