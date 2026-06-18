mod parse;

use proc_macro::TokenStream;
use syn::parse2;

use crate::parse::MacroSnapshot;

#[proc_macro]
pub fn snapshot_delegate_components(body: TokenStream) -> TokenStream {
    let snapshot: MacroSnapshot = parse2(body.into()).unwrap();

    let output = cgp_macro_lib::delegate_components(snapshot.body.clone()).unwrap();

    snapshot.wrap_output(output).into()
}
