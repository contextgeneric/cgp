use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{FnArg, ImplItem, ImplItemFn, ItemImpl};

use crate::cgp_fn::{ImplicitArgField, extract_implicit_args, parse_implicit_arg};

pub fn extract_implicit_args_from_item_impl(item_impl: &mut ItemImpl) {
    for item in item_impl.items.iter_mut() {
        if let ImplItem::Fn(method) = item {}
    }
}
