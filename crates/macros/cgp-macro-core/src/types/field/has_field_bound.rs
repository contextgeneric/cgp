use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Type;
use syn::token::Mut;

use crate::exports::{HasField, HasFieldMut};
use crate::types::getter::FieldMode;

pub struct HasFieldBound {
    pub field_type: Type,
    pub field_mut: Option<Mut>,
    pub field_mode: FieldMode,
    pub tag_type: Type,
}

impl ToTokens for HasFieldBound {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            field_type,
            field_mut,
            field_mode,
            tag_type,
        } = self;

        let output = if field_mut.is_none() {
            if let FieldMode::Slice = field_mode {
                quote! {
                    #HasField< #tag_type, Value: AsRef< [ #field_type ] > + 'static >
                }
            } else {
                quote! {
                    #HasField< #tag_type, Value = #field_type >
                }
            }
        } else {
            quote! {
                #HasFieldMut< #tag_type, Value = #field_type >
            }
        };

        tokens.extend(output);
    }
}
