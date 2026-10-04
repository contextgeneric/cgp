use cgp_macro_core::functions::to_camel_case_str;
use syn::Ident;

/// The default name of a provider defined from a function: the function name in
/// PascalCase, so `magic_number` names `MagicNumber`. The name is spanned on the
/// function identifier it derives from, so an error on the generated struct, and
/// an editor's go-to-definition, both land on that identifier.
pub fn derive_provider_ident(fn_ident: &Ident) -> Ident {
    Ident::new(&to_camel_case_str(&fn_ident.to_string()), fn_ident.span())
}
