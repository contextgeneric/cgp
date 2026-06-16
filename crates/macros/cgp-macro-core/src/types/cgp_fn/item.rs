use syn::{Ident, ItemFn, Visibility};

use crate::functions::{extract_and_parse_implicit_args, to_camel_case_str};
use crate::types::attributes::FunctionAttributes;
use crate::types::cgp_fn::PreprocessedItemCgpFn;

pub struct ItemCgpFn {
    pub ident: Option<Ident>,
    pub item_fn: ItemFn,
}

impl ItemCgpFn {
    pub fn preprocess(&self) -> syn::Result<PreprocessedItemCgpFn> {
        let mut item_fn = self.item_fn.clone();

        let ident = self.ident.clone().unwrap_or_else(|| {
            Ident::new(
                &to_camel_case_str(&item_fn.sig.ident.to_string()),
                item_fn.sig.ident.span(),
            )
        });

        let visibility = item_fn.vis.clone();
        item_fn.vis = Visibility::Inherited;

        let implicit_args = extract_and_parse_implicit_args(&mut item_fn.sig.inputs)?;
        implicit_args.prepend_to_block(&mut item_fn.block)?;

        let attributes = FunctionAttributes::parse(core::mem::take(&mut item_fn.attrs))?;

        let generics = core::mem::take(&mut item_fn.sig.generics);

        Ok(PreprocessedItemCgpFn {
            ident,
            item_fn,
            implicit_args,
            attributes,
            visibility,
            generics,
        })
    }
}
