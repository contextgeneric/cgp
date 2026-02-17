use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, ItemFn, ItemImpl, ItemTrait, TraitItemFn, Visibility, parse2};

use crate::cgp_fn::extract_implicits_args;
use crate::cgp_fn::fn_body::inject_implicit_args;
use crate::derive_getter::derive_getter_constraint;
use crate::symbol::symbol_from_string;

pub fn derive_cgp_fn(trait_ident: &Ident, mut item_fn: ItemFn) -> syn::Result<TokenStream> {
    match item_fn.sig.inputs.first() {
        Some(syn::FnArg::Receiver(_)) => {}
        _ => {
            return Err(syn::Error::new_spanned(
                &item_fn.sig.inputs,
                "First argument must be self",
            ));
        }
    }

    let implicit_args = extract_implicits_args(&mut item_fn.sig.inputs)?;

    item_fn.vis = Visibility::Inherited;

    let trait_item_fn = TraitItemFn {
        attrs: item_fn.attrs.clone(),
        sig: item_fn.sig.clone(),
        default: None,
        semi_token: None,
    };

    inject_implicit_args(&implicit_args, &mut item_fn.block)?;

    let item_trait: ItemTrait = parse2(quote! {
        pub trait #trait_ident {
            #trait_item_fn
        }
    })?;

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

    let output = quote! {
        #item_trait

        #item_impl

    };

    Ok(output)
}
