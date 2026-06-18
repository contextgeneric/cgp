use cgp_macro_test_util_lib::entrypoints;
use proc_macro::TokenStream;

#[proc_macro]
pub fn snapshot_delegate_components(body: TokenStream) -> TokenStream {
    entrypoints::snapshot_delegate_components(body.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

// #[proc_macro]
// pub fn snapshot_cgp_component(body: TokenStream) -> TokenStream {
//     let snapshot: MacroSnapshot = parse2(body.into()).unwrap();

//     let output = cgp_macro_lib::cgp_component(
//         snapshot.attrs.clone().unwrap().into(),
//         snapshot.body.clone(),
//     )
//     .unwrap();

//     snapshot.wrap_output(output).into()
// }
