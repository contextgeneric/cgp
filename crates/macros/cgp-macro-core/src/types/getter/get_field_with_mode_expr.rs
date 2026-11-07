use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::types::getter::{FieldMode, GetFieldExpr};

/// A field read with its conversion applied.
pub struct GetFieldWithModeExpr {
    /// The field read.
    pub get_field: GetFieldExpr,
    /// The conversion to apply.
    pub field_mode: FieldMode,
}

impl ToTokens for GetFieldWithModeExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field_mut = self.get_field.field_mut;
        let expr = self
            .field_mode
            .apply(self.get_field.to_token_stream(), &field_mut);

        tokens.extend(expr);
    }
}
