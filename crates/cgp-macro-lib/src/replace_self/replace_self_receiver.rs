use proc_macro2::{Ident, TokenStream};
use syn::{FnArg, Receiver, Signature, parse_quote};

use crate::replace_self::parse_and_replace_self_type;

pub fn replace_self_receiver_in_signature(
    sig: &mut Signature,
    replaced_var: &Ident,
    replaced_type: TokenStream,
) -> syn::Result<()> {
    if let Some(arg) = sig.inputs.first_mut()
        && let FnArg::Receiver(receiver) = arg
    {
        *arg = replace_self_receiver(receiver, replaced_var, replaced_type)?;
    }

    Ok(())
}

pub fn replace_self_receiver(
    receiver: &mut Receiver,
    replaced_var: &Ident,
    replaced_type: TokenStream,
) -> syn::Result<FnArg> {
    let new_type = parse_and_replace_self_type(&receiver.ty, replaced_type, &Vec::new())?;

    Ok(parse_quote!(#replaced_var : #new_type))
}
