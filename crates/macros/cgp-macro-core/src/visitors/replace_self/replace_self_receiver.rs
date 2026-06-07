use proc_macro2::Ident;
use syn::visit_mut::VisitMut;
use syn::{FnArg, Receiver, Signature, Type, parse_quote};

pub struct ReplaceSelfReceiverVisitor<'a> {
    pub replaced_ident: &'a Ident,
    pub replaced_type: &'a Type,
}

impl<'a> VisitMut for ReplaceSelfReceiverVisitor<'a> {
    fn visit_signature_mut(&mut self, sig: &mut Signature) {
        if let Some(arg) = sig.inputs.first_mut()
            && let FnArg::Receiver(receiver) = arg
        {
            *arg = replace_self_receiver(receiver, self.replaced_ident, self.replaced_type);
        }
    }
}

pub fn replace_self_receiver(
    receiver: &mut Receiver,
    replaced_ident: &Ident,
    replaced_type: &Type,
) -> FnArg {
    match (&receiver.reference, &receiver.mutability) {
        (None, None) => {
            parse_quote!(#replaced_ident : #replaced_type)
        }
        (Some((_and, None)), None) => {
            parse_quote!(#replaced_ident : & #replaced_type)
        }
        (Some((_and, Some(life))), None) => {
            parse_quote!(#replaced_ident : & #life #replaced_type)
        }
        (Some((_and, None)), Some(_mut)) => {
            parse_quote!(#replaced_ident : &mut #replaced_type)
        }
        (Some((_and, Some(life))), Some(_mut)) => {
            parse_quote!(#replaced_ident : & #life mut #replaced_type)
        }
        (None, Some(_mut)) => {
            parse_quote!(#replaced_ident : mut #replaced_type)
        }
    }
}
