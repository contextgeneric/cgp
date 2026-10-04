use cgp_macro_core::functions::to_camel_case_str;
use syn::Ident;
use syn::ext::IdentExt;

/// The default name of a provider defined from a function: the function name in
/// PascalCase, so `magic_number` names `MagicNumber`. The name is spanned on the
/// function identifier it derives from, so an error on the generated struct, and
/// an editor's go-to-definition, both land on that identifier.
pub fn derive_provider_ident(fn_ident: &Ident) -> Ident {
    Ident::new(&to_pascal_case(fn_ident), fn_ident.span())
}

/// The name of a dispatch method's per-variant computer: `Compute` plus the
/// method name in PascalCase, spanned on the method identifier.
pub fn derive_computer_ident(method_ident: &Ident) -> Ident {
    Ident::new(
        &format!("Compute{}", to_pascal_case(method_ident)),
        method_ident.span(),
    )
}

/// `ident` in PascalCase. A raw identifier is unrawed first, since `r#` cannot
/// begin a longer identifier: `r#type` yields `Type`.
fn to_pascal_case(ident: &Ident) -> String {
    to_camel_case_str(&ident.unraw().to_string())
}
