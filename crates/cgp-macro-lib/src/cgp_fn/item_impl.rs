use quote::{ToTokens, quote};
use syn::{Ident, ItemFn, ItemImpl, parse2};

use crate::cgp_fn::ImplicitArgField;
use crate::derive_getter::derive_getter_constraint;
use crate::symbol::symbol_from_string;

pub fn derive_item_impl(
    trait_ident: &Ident,
    item_fn: &ItemFn,
    implicit_args: &[ImplicitArgField],
) -> syn::Result<ItemImpl> {
    let mut item_impl: ItemImpl = parse2(quote! {
        impl<__Context__> #trait_ident for __Context__ {
            #item_fn
        }
    })?;

    let where_clause = item_impl.generics.make_where_clause();

    for arg in implicit_args {
        let field_symbol = symbol_from_string(&arg.field_name.to_string());

        let constraint = derive_getter_constraint(
            &arg.field_type,
            &arg.field_mut,
            &arg.field_mode,
            field_symbol.to_token_stream(),
            &None,
        )?;

        where_clause.predicates.push(parse2(quote! {
            Self: #constraint
        })?);
    }

    Ok(item_impl)
}
