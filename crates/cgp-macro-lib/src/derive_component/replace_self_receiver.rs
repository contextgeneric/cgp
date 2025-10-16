use proc_macro2::{Ident, TokenStream};
use syn::{FnArg, Receiver, Signature, parse_quote};

pub fn replace_self_receiver_in_signature(
    sig: &mut Signature,
    replaced_var: &Ident,
    replaced_type: TokenStream,
) {
    if let Some(arg) = sig.inputs.first_mut()
        && let FnArg::Receiver(receiver) = arg
    {
        *arg = replace_self_receiver(receiver, replaced_var, replaced_type);
    }
}

pub fn replace_self_receiver(
    receiver: &mut Receiver,
    replaced_var: &Ident,
    replaced_type: TokenStream,
) -> FnArg {
    match (&receiver.reference, &receiver.mutability) {
        (None, None) => {
            parse_quote!(#replaced_var : #replaced_type)
        }
        (Some((_and, None)), None) => {
            parse_quote!(#replaced_var : & #replaced_type)
        }
        (Some((_and, Some(life))), None) => {
            parse_quote!(#replaced_var : & #life #replaced_type)
        }
        (Some((_and, None)), Some(_mut)) => {
            parse_quote!(#replaced_var : &mut #replaced_type)
        }
        (Some((_and, Some(life))), Some(_mut)) => {
            parse_quote!(#replaced_var : & #life mut #replaced_type)
        }
        (None, Some(_mut)) => {
            parse_quote!(#replaced_var : mut #replaced_type)
        }
    }
}
