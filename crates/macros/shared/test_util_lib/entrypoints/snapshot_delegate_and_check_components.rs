use proc_macro2::TokenStream;
use syn::parse2;

use crate::test_util_lib::keywords::DelegateAndCheckComponents;
use crate::test_util_lib::types::StatementMacroSnapshot;

pub fn snapshot_delegate_and_check_components(body: TokenStream) -> syn::Result<TokenStream> {
    let item: StatementMacroSnapshot<DelegateAndCheckComponents> = parse2(body)?;

    let output = crate::macro_lib::delegate_and_check_components(item.body.clone())?;

    item.snapshot.wrap_output(output)
}
