use syn::ImplItem;

use crate::functions::extract_and_parse_implicit_args;
use crate::types::implicits::ImplicitArgFields;

pub fn extract_implicit_args_from_impl_items(
    impl_items: &mut [ImplItem],
) -> syn::Result<ImplicitArgFields> {
    let mut all_fields = Vec::new();

    for item in impl_items {
        if let ImplItem::Fn(method) = item {
            let implicit_args = extract_and_parse_implicit_args(&mut method.sig.inputs)?;
            implicit_args.prepend_to_block(&mut method.block)?;

            for implicit_arg in implicit_args.fields {
                if !all_fields.contains(&implicit_arg) {
                    all_fields.push(implicit_arg);
                }
            }
        }
    }

    Ok(ImplicitArgFields { fields: all_fields })
}
