use cgp_macro_test_util_lib::types::MacroSnapshot;
use proc_macro::TokenStream;
use syn::parse2;

#[proc_macro]
pub fn snapshot_delegate_components(body: TokenStream) -> TokenStream {
    let snapshot: MacroSnapshot = parse2(body.into()).unwrap();

    let output = cgp_macro_lib::delegate_components(snapshot.body.clone()).unwrap();

    snapshot.wrap_output(output).into()
}

#[proc_macro]
pub fn snapshot_cgp_component(body: TokenStream) -> TokenStream {
    let snapshot: MacroSnapshot = parse2(body.into()).unwrap();

    let output = cgp_macro_lib::cgp_component(
        snapshot.attrs.clone().unwrap().into(),
        snapshot.body.clone(),
    )
    .unwrap();

    snapshot.wrap_output(output).into()
}
