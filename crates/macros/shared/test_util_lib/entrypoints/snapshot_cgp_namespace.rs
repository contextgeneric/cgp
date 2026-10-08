use proc_macro2::TokenStream;
use syn::parse2;

use crate::test_util_lib::keywords::CgpNamespace;
use crate::test_util_lib::types::StatementMacroSnapshot;

pub fn snapshot_cgp_namespace(body: TokenStream) -> syn::Result<TokenStream> {
    let item: StatementMacroSnapshot<CgpNamespace> = parse2(body)?;

    let output = crate::macro_lib::cgp_namespace(item.body.clone())?;

    item.snapshot.wrap_output(output)
}
