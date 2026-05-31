use cgp_macro_core::types::implicits::ImplicitArgField;
use syn::ImplItem;

use crate::cgp_fn::extract_and_parse_implicit_args;

pub fn extract_implicit_args_from_impl_items(
    impl_items: &mut [ImplItem],
) -> syn::Result<Vec<ImplicitArgField>> {
    let mut all_implicit_args = Vec::new();

    for item in impl_items {
        if let ImplItem::Fn(method) = item {
            let implicit_args = extract_and_parse_implicit_args(&mut method.sig.inputs)?;
            implicit_args.prepend_to_block(&mut method.block)?;

            for implicit_arg in implicit_args.fields {
                if !all_implicit_args.contains(&implicit_arg) {
                    all_implicit_args.push(implicit_arg);
                }
            }
        }
    }

    Ok(all_implicit_args)
}
