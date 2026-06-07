use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::types::getter::{FieldMode, GetFieldExpr};

pub struct GetFieldWithModeExpr {
    pub get_field: GetFieldExpr,
    pub field_mode: FieldMode,
}

impl ToTokens for GetFieldWithModeExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let call_expr = &self.get_field;

        let expr = match self.field_mode {
            FieldMode::Reference => {
                quote!(#call_expr)
            }
            FieldMode::OptionRef => {
                if call_expr.field_mut.is_none() {
                    quote! {
                        #call_expr .as_ref()
                    }
                } else {
                    quote! {
                        #call_expr .as_mut()
                    }
                }
            }
            FieldMode::MRef => {
                quote! {
                    MRef::Ref( #call_expr )
                }
            }
            FieldMode::Str => {
                if call_expr.field_mut.is_none() {
                    quote! {
                        #call_expr .as_str()
                    }
                } else {
                    quote! {
                        #call_expr .as_mut_str()
                    }
                }
            }
            FieldMode::Copy => {
                quote! {
                    #call_expr .clone()
                }
            }
            FieldMode::Slice => {
                quote! {
                    #call_expr .as_ref()
                }
            }
        };

        tokens.extend(expr);
    }
}
