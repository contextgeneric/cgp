use proc_macro2::TokenStream;
use syn::parse2;

use crate::test_util_lib::keywords::DelegateComponents;
use crate::test_util_lib::types::StatementMacroSnapshot;

pub fn snapshot_delegate_components(body: TokenStream) -> syn::Result<TokenStream> {
    let item: StatementMacroSnapshot<DelegateComponents> = parse2(body)?;

    let output = crate::macro_lib::delegate_components(item.body.clone())?;

    item.snapshot.wrap_output(output)
}
