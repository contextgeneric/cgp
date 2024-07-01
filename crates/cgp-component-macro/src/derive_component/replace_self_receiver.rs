use proc_macro2::Ident;
use syn::{parse_quote, FnArg, TraitItemFn};

use crate::derive_component::snake_case::to_snake_case;

pub fn replace_self_receiver(func: &mut TraitItemFn, replaced_type: &Ident) {
    if let Some(arg) = func.sig.inputs.first_mut() {
        if let FnArg::Receiver(receiver) = arg {
            let replaced_var = to_snake_case(replaced_type);

            match (&receiver.reference, &receiver.mutability) {
                (None, None) => {
                    *arg = parse_quote!(#replaced_var : #replaced_type);
                }
                (Some((_and, None)), None) => {
                    *arg = parse_quote!(#replaced_var : & #replaced_type);
                }
                (Some((_and, Some(life))), None) => {
                    *arg = parse_quote!(#replaced_var : & #life #replaced_type);
                }
                (Some((_and, None)), Some(_mut)) => {
                    *arg = parse_quote!(#replaced_var : &mut #replaced_type);
                }
                (Some((_and, Some(life))), Some(_mut)) => {
                    *arg = parse_quote!(#replaced_var : & #life mut #replaced_type);
                }
                _ => {}
            }
        }
    }
}
