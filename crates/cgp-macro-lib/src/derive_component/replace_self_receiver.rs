use proc_macro2::{Ident, TokenStream};
use syn::{FnArg, Signature, parse_quote};

pub fn replace_self_receiver(
    sig: &mut Signature,
    replaced_var: &Ident,
    replaced_type: TokenStream,
) {
    if let Some(arg) = sig.inputs.first_mut()
        && let FnArg::Receiver(receiver) = arg
    {
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
